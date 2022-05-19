//! Module for error handling code.

use thiserror::Error;

#[derive(Debug, Error)]
#[allow(dead_code)] // XXX FIXME
pub enum BmputilError
{
    #[error("Failed to read firmware file {filename}")]
    FirmwareFileIOError
    {
        #[source]
        source: std::io::Error,
        filename: String,
    },

    #[error("More than one Blackmagic Probe device was found")]
    TooManyDevicesError,

    #[error("No connected Blackmagic Probe device was found! Check connection?")]
    DeviceNotFoundError,

    #[error("Access denied when attempting to {operation} to {context}")]
    PermissionsError
    {
        #[source]
        source: rusb::Error,

        /// The USB/libusb operation that failed (e.g. `"send a control transfer"`).
        operation: String,

        /// The context that operation was being performed in (e.g.: `"read firmware version"`).
        context: String,
    },

    #[error("Blackmagic Probe device found disconnected when attempting to {operation} to {context}")]
    DeviceDisconnectDuringOperationError
    {
        #[source]
        source: rusb::Error,

        /// The USB/libusb operation that failed (e.g.: `"send a control transfer"`).
        operation: String,

        /// The context that operation was being performed in (e.g.: `"read firmware version"`).
        context: String,
    },

    #[error("Blackmagic Probe device did not re-enumerate after requesting to switch to DFU mode")]
    DeviceReconfigureError
    {
        /// Source is optional because there may be no libusb error, if detecting connection is
        /// done through e.g. checking device lists.
        #[source]
        source: Option<rusb::Error>,
    },

    #[allow(dead_code)] // FIXME: this will presumably be used once we, well, actually implement the post-flash check.
    #[error("Blackmagic Probe device did not re-enumerate after flashing firmware; firmware may be invalid?")]
    DeviceRebootError
    {
        #[source]
        source: Option<rusb::Error>,
    },


    #[error(
        "Blackmagic Probe device returned bad data ({invalid_thing}) during configuration.\
        This generally shouldn't be possible. Maybe cable is bad, or OS is messing with things?"
    )]
    DeviceSeemsInvalidError
    {
        #[source]
        source: Option<anyhow::Error>,
        invalid_thing: String,
    },

    #[error("Other/unhandled libusb error (please report this so we can add better handling!)")]
    LibusbError(#[from] rusb::Error),

    #[error("Other/unhandled dfu_libusb error (please report this so we can add better error handling!")]
    DfuLibusbError(#[from] dfu_libusb::Error),
}


#[macro_export]
macro_rules! log_and_return
{
    ($err:expr) => {
        let err = $err;
        log::error!("{}", err);
        return Err(err);
    }
}
