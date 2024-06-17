/// Control how the offset within the file for each string will be displayed, if desired.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocationRadix {
    Octal,
    Decimal,
    Hex,
}

/// Control the encoding of the strings that are found.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Encoding {
    /// 7-bit byte characters
    #[default]
    SevenBit,

    /// 8-bit byte characters
    EightBit,

    /// 16-bit big endian
    SixteenBitBE,

    /// 16-bit little endian
    SixteenBitLE,

    /// 32-bit big endian
    ThirtyTwoBitBE,

    /// 32-bit little endian
    ThirtyTwoBitLE,
}

/// Controls how UTF-8 encoded Unicode characters are displayed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UnicodeDisplay {
    /// Rely on the setting of --encoding
    #[default]
    RelyOnEncoding,

    /// Treats as non-graphic, invalid characters
    Invalid,

    /// Displays in current locale
    Locale,

    /// Displays as escape sequences
    Escape,

    /// Displays as hex byte sequences enclosed between <>.
    Hex,

    /// Display in red highlight, if supported
    Highlight,
}

#[derive(Debug)]
pub struct Config {
    /// Only scan data sections in file. (scans whole file by default)
    pub data_only: bool,

    /// Print the file name before each string.
    pub print_file_name: bool,

    /// Minimum displayable character sequence length.
    pub min_seq_len: u16,

    /// Location radix.
    pub loc_radix: Option<LocationRadix>,

    /// Include whitespace as valid string characters.
    pub whitespace: bool,

    /// Character encoding.
    pub char_encoding: Encoding,

    /// Unicode display.
    pub unicode_display: UnicodeDisplay,

    /// String used to separate strings in output.
    pub separator: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            data_only: false,
            print_file_name: false,
            min_seq_len: 4,
            loc_radix: None,
            whitespace: false,
            char_encoding: Encoding::default(),
            unicode_display: UnicodeDisplay::default(),
            separator: String::new(),
        }
    }
}

#[derive(Debug, Default)]
pub struct ConfigBuilder {
    config: Config,
}

impl ConfigBuilder {
    pub fn data_only(mut self, data_only: bool) -> Self {
        self.config.data_only = data_only;
        self
    }

    pub fn print_file_name(mut self, print_file_name: bool) -> Self {
        self.config.print_file_name = print_file_name;
        self
    }

    pub fn min_seq_len(mut self, min_seq_len: u16) -> Self {
        self.config.min_seq_len = min_seq_len;
        self
    }

    pub fn loc_radix(mut self, loc_radix: LocationRadix) -> Self {
        self.config.loc_radix = Some(loc_radix);
        self
    }

    pub fn whitespace(mut self, whitespace: bool) -> Self {
        self.config.whitespace = whitespace;
        self
    }

    pub fn char_encoding(mut self, char_encoding: Encoding) -> Self {
        self.config.char_encoding = char_encoding;
        self
    }

    pub fn unicode_display(mut self, unicode_display: UnicodeDisplay) -> Self {
        self.config.unicode_display = unicode_display;
        self
    }

    pub fn separator(mut self, separator: String) -> Self {
        self.config.separator = separator;
        self
    }

    pub fn build(self) -> Config {
        self.config
    }
}
