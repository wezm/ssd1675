use command::{BufCommand, Command, DataEntryMode, IncrementAxis};
use display::{self, Dimensions, Rotation};

/// Builder for constructing a display Config.
///
/// Dimensions must supplied, all other settings will use a default value if not supplied. However
/// it's likely that LUT values will need to be supplied to successfully use a display.
///
/// ### Example
///
/// ```
/// use ssd1675::{Builder, Dimensions, Rotation};
///
/// let config = Builder::new()
///     .dimensions(Dimensions {
///         rows: 212,
///         cols: 104,
///     })
///     .rotation(Rotation::Rotate270)
///     .build()
///     .expect("invalid configuration");
/// ```
pub struct Builder<'a> {
    dummy_line_period: Command,
    gate_line_width: Command,
    write_vcom: Command,
    write_lut: Option<BufCommand<'a>>,
    write_color: Option<Command>,
    data_entry_mode: Command,
    dimensions: Option<Dimensions>,
    rotation: Rotation,
}

/// Error returned if Builder configuration is invalid.
///
/// Currently only returned if a configuration is built without dimensions.
#[derive(Debug)]
pub struct BuilderError {}

/// Display configuration.
///
/// Passed to Display::new. Use `Builder` to construct a `Config`.
pub struct Config<'a> {
    pub(crate) dummy_line_period: Command,
    pub(crate) gate_line_width: Command,
    pub(crate) write_vcom: Command,
    pub(crate) write_color: Option<Command>,
    pub(crate) write_lut: Option<BufCommand<'a>>,
    pub(crate) data_entry_mode: Command,
    pub(crate) dimensions: Dimensions,
    pub(crate) rotation: Rotation,
}

impl<'a> Default for Builder<'a> {
    fn default() -> Self {
        Builder {
            dummy_line_period: Command::DummyLinePeriod(0x07),
            gate_line_width: Command::GateLineWidth(0x04),
            write_vcom: Command::WriteVCOM(0x3C),
            write_lut: None,
            write_color: None,
            data_entry_mode: Command::DataEntryMode(
                DataEntryMode::IncrementYIncrementX,
                IncrementAxis::Horizontal,
            ),
            dimensions: None,
            rotation: Rotation::default(),
        }
    }
}

impl<'a> Builder<'a> {
    /// Create a new Builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the number of dummy line period in terms of gate line width (TGate).
    ///
    /// Defaults to 0x07. Corresponds to command 0x3A.
    pub fn dummy_line_period(self, dummy_line_period: u8) -> Self {
        Self {
            dummy_line_period: Command::DummyLinePeriod(dummy_line_period),
            ..self
        }
    }

    /// Set the gate line width (TGate).
    ///
    /// Defaults to 0x04. Corresponds to command 0x3B.
    pub fn gate_line_width(self, gate_line_width: u8) -> Self {
        Self {
            gate_line_width: Command::GateLineWidth(gate_line_width),
            ..self
        }
    }

    /// Set VCOM register value.
    ///
    /// Defaults to 0x3C. Corresponds to command 0x2C.
    pub fn vcom(self, value: u8) -> Self {
        Self {
            write_vcom: Command::WriteVCOM(value),
            ..self
        }
    }

    /// Set lookup table (70 bytes).
    ///
    /// **Note:** The supplied slice must be exactly 70 bytes long.
    ///
    /// There is no default for the lookup table. Corresponds to command 0x32. If not supplied then
    /// the default in the controller is used. Apparently the display manufacturer will normally
    /// supply the LUT values for a particular display batch.
    pub fn lut(self, lut: &'a [u8]) -> Self {
        Self {
            write_lut: Some(BufCommand::WriteLUT(lut)),
            ..self
        }
    }

    /// Set color driving voltage
    pub fn yellow(self, yellow: &'a bool) -> Self {
        if ! yellow {
           return self
        }
        Self {
            write_color: Some(Command::SourceDrivingVoltageYellow(0x07)),
            ..self
        }
    }

    /// Define data entry sequence.
    ///
    /// Defaults to DataEntryMode::IncrementAxis, IncrementAxis::Horizontal. Corresponds to command
    /// 0x11.
    pub fn data_entry_mode(
        self,
        data_entry_mode: DataEntryMode,
        increment_axis: IncrementAxis,
    ) -> Self {
        Self {
            data_entry_mode: Command::DataEntryMode(data_entry_mode, increment_axis),
            ..self
        }
    }

    /// Set the display dimensions.
    ///
    /// There is no default for this setting. The dimensions must be set for the builder to
    /// successfully build a Config.
    pub fn dimensions(self, dimensions: Dimensions) -> Self {
        assert!(
            dimensions.cols % 8 == 0,
            "columns must be evenly divisible by 8"
        );
        assert!(
            dimensions.rows <= display::MAX_GATE_OUTPUTS,
            "rows must be less than MAX_GATE_OUTPUTS"
        );
        assert!(
            dimensions.cols <= display::MAX_SOURCE_OUTPUTS,
            "cols must be less than MAX_SOURCE_OUTPUTS"
        );

        Self {
            dimensions: Some(dimensions),
            ..self
        }
    }

    /// Set the display rotation.
    ///
    /// Defaults to no rotation (`Rotation::Rotate0`). Use this to translate between the physical
    /// rotation of the display and how the data is displayed on the display.
    pub fn rotation(self, rotation: Rotation) -> Self {
        Self { rotation, ..self }
    }

    /// Build the display Config.
    ///
    /// Will fail if dimensions are not set.
    pub fn build(self) -> Result<Config<'a>, BuilderError> {
        Ok(Config {
            dummy_line_period: self.dummy_line_period,
            gate_line_width: self.gate_line_width,
            write_vcom: self.write_vcom,
            write_lut: self.write_lut,
            write_color: self.write_color,
            data_entry_mode: self.data_entry_mode,
            dimensions: self.dimensions.ok_or_else(|| BuilderError {})?,
            rotation: self.rotation,
        })
    }
}
