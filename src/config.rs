use command::{BufCommand, Command, DataEntryMode, IncrementAxis};
use display::{Dimensions, Rotation};

pub struct Builder<'a> {
    dummy_line_period: Command,
    gate_line_width: Command,
    write_vcom: Command,
    write_lut: Option<BufCommand<'a>>,
    data_entry_mode: Command,
    dimensions: Option<Dimensions>,
    rotation: Rotation,
}

#[derive(Debug)]
pub struct BuilderError {}

pub struct Config<'a> {
    pub(crate) dummy_line_period: Command,
    pub(crate) gate_line_width: Command,
    pub(crate) write_vcom: Command,
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
    pub fn new() -> Self {
        Self::default()
    }

    pub fn dummy_line_period(self, dummy_line_period: u8) -> Self {
        Self {
            dummy_line_period: Command::DummyLinePeriod(dummy_line_period),
            ..self
        }
    }

    pub fn gate_line_width(self, gate_line_width: u8) -> Self {
        Self {
            gate_line_width: Command::GateLineWidth(gate_line_width),
            ..self
        }
    }

    pub fn vcom(self, value: u8) -> Self {
        Self {
            write_vcom: Command::WriteVCOM(value),
            ..self
        }
    }

    pub fn lut(self, lut: &'a [u8]) -> Self {
        Self {
            write_lut: Some(BufCommand::WriteLUT(lut)),
            ..self
        }
    }

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

    pub fn dimensions(self, dimensions: Dimensions) -> Self {
        Self {
            dimensions: Some(dimensions),
            ..self
        }
    }

    pub fn rotation(self, rotation: Rotation) -> Self {
        Self { rotation, ..self }
    }

    pub fn build(self) -> Result<Config<'a>, BuilderError> {
        Ok(Config {
            dummy_line_period: self.dummy_line_period,
            gate_line_width: self.gate_line_width,
            write_vcom: self.write_vcom,
            write_lut: self.write_lut,
            data_entry_mode: self.data_entry_mode,
            dimensions: self.dimensions.ok_or_else(|| BuilderError {})?,
            rotation: self.rotation,
        })
    }
}
