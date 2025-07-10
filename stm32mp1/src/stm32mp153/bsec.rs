#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    otp_config: OTP_CONFIG,
    otp_control: OTP_CONTROL,
    otp_wrdata: OTP_WRDATA,
    otp_status: OTP_STATUS,
    otp_lock: OTP_LOCK,
    denable: DENABLE,
    _reserved6: [u8; 0x04],
    otp_disturbed0: OTP_DISTURBED0,
    otp_disturbed1: OTP_DISTURBED1,
    otp_disturbed2: OTP_DISTURBED2,
    _reserved9: [u8; 0x0c],
    otp_error0: OTP_ERROR0,
    otp_error1: OTP_ERROR1,
    otp_error2: OTP_ERROR2,
    _reserved12: [u8; 0x0c],
    otp_wrlock0: OTP_WRLOCK0,
    otp_wrlock1: OTP_WRLOCK1,
    otp_wrlock2: OTP_WRLOCK2,
    _reserved15: [u8; 0x0c],
    otp_splock0: OTP_SPLOCK0,
    otp_splock1: OTP_SPLOCK1,
    otp_splock2: OTP_SPLOCK2,
    _reserved18: [u8; 0x0c],
    otp_swlock0: OTP_SWLOCK0,
    otp_swlock1: OTP_SWLOCK1,
    otp_swlock2: OTP_SWLOCK2,
    _reserved21: [u8; 0x0c],
    otp_srlock0: OTP_SRLOCK0,
    otp_srlock1: OTP_SRLOCK1,
    otp_srlock2: OTP_SRLOCK2,
    _reserved24: [u8; 0x0c],
    jtagin: JTAGIN,
    jtagout: JTAGOUT,
    scratch: SCRATCH,
    _reserved27: [u8; 0x0148],
    otp_data0: OTP_DATA0,
    otp_data1: OTP_DATA1,
    otp_data2: OTP_DATA2,
    otp_data3: OTP_DATA3,
    otp_data4: OTP_DATA4,
    otp_data5: OTP_DATA5,
    otp_data6: OTP_DATA6,
    otp_data7: OTP_DATA7,
    otp_data8: OTP_DATA8,
    otp_data9: OTP_DATA9,
    otp_data10: OTP_DATA10,
    otp_data11: OTP_DATA11,
    otp_data12: OTP_DATA12,
    otp_data13: OTP_DATA13,
    otp_data14: OTP_DATA14,
    otp_data15: OTP_DATA15,
    otp_data16: OTP_DATA16,
    otp_data17: OTP_DATA17,
    otp_data18: OTP_DATA18,
    otp_data19: OTP_DATA19,
    otp_data20: OTP_DATA20,
    otp_data21: OTP_DATA21,
    otp_data22: OTP_DATA22,
    otp_data23: OTP_DATA23,
    otp_data24: OTP_DATA24,
    otp_data25: OTP_DATA25,
    otp_data26: OTP_DATA26,
    otp_data27: OTP_DATA27,
    otp_data28: OTP_DATA28,
    otp_data29: OTP_DATA29,
    otp_data30: OTP_DATA30,
    otp_data31: OTP_DATA31,
    otp_data32: OTP_DATA32,
    otp_data33: OTP_DATA33,
    otp_data34: OTP_DATA34,
    otp_data35: OTP_DATA35,
    otp_data36: OTP_DATA36,
    otp_data37: OTP_DATA37,
    otp_data38: OTP_DATA38,
    otp_data39: OTP_DATA39,
    otp_data40: OTP_DATA40,
    otp_data41: OTP_DATA41,
    otp_data42: OTP_DATA42,
    otp_data43: OTP_DATA43,
    otp_data44: OTP_DATA44,
    otp_data45: OTP_DATA45,
    otp_data46: OTP_DATA46,
    otp_data47: OTP_DATA47,
    otp_data48: OTP_DATA48,
    otp_data49: OTP_DATA49,
    otp_data50: OTP_DATA50,
    otp_data51: OTP_DATA51,
    otp_data52: OTP_DATA52,
    otp_data53: OTP_DATA53,
    otp_data54: OTP_DATA54,
    otp_data55: OTP_DATA55,
    otp_data56: OTP_DATA56,
    otp_data57: OTP_DATA57,
    otp_data58: OTP_DATA58,
    otp_data59: OTP_DATA59,
    otp_data60: OTP_DATA60,
    otp_data61: OTP_DATA61,
    otp_data62: OTP_DATA62,
    otp_data63: OTP_DATA63,
    otp_data64: OTP_DATA64,
    otp_data65: OTP_DATA65,
    otp_data66: OTP_DATA66,
    otp_data67: OTP_DATA67,
    otp_data68: OTP_DATA68,
    otp_data69: OTP_DATA69,
    otp_data70: OTP_DATA70,
    otp_data71: OTP_DATA71,
    otp_data72: OTP_DATA72,
    otp_data73: OTP_DATA73,
    otp_data74: OTP_DATA74,
    otp_data75: OTP_DATA75,
    otp_data76: OTP_DATA76,
    otp_data77: OTP_DATA77,
    otp_data78: OTP_DATA78,
    otp_data79: OTP_DATA79,
    otp_data80: OTP_DATA80,
    otp_data81: OTP_DATA81,
    otp_data82: OTP_DATA82,
    otp_data83: OTP_DATA83,
    otp_data84: OTP_DATA84,
    otp_data85: OTP_DATA85,
    otp_data86: OTP_DATA86,
    otp_data87: OTP_DATA87,
    otp_data88: OTP_DATA88,
    otp_data89: OTP_DATA89,
    otp_data90: OTP_DATA90,
    otp_data91: OTP_DATA91,
    otp_data92: OTP_DATA92,
    otp_data93: OTP_DATA93,
    otp_data94: OTP_DATA94,
    otp_data95: OTP_DATA95,
    _reserved123: [u8; 0x0c70],
    hwcfgr: HWCFGR,
    verr: VERR,
    ipidr: IPIDR,
    sidr: SIDR,
}
impl RegisterBlock {
    ///0x00 - BSEC OTP configuration register
    #[inline(always)]
    pub const fn otp_config(&self) -> &OTP_CONFIG {
        &self.otp_config
    }
    ///0x04 - BSEC OTP control register
    #[inline(always)]
    pub const fn otp_control(&self) -> &OTP_CONTROL {
        &self.otp_control
    }
    ///0x08 - BSEC OTP write data register
    #[inline(always)]
    pub const fn otp_wrdata(&self) -> &OTP_WRDATA {
        &self.otp_wrdata
    }
    ///0x0c - BSEC OTP status register
    #[inline(always)]
    pub const fn otp_status(&self) -> &OTP_STATUS {
        &self.otp_status
    }
    ///0x10 - BSEC OTP lock configuration register
    #[inline(always)]
    pub const fn otp_lock(&self) -> &OTP_LOCK {
        &self.otp_lock
    }
    ///0x14 - reset value depends on OTP secure mode according toTable18: BSEC_DENABLE default values after reset on page181.
    #[inline(always)]
    pub const fn denable(&self) -> &DENABLE {
        &self.denable
    }
    ///0x1c - BSEC_OTP_DISTURBED0 is used to report disturbed state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP). BSEC_OTP_DISTURBED1 is used to report disturbed state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_DISTURBED2 is used to report disturbed state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95.
    #[inline(always)]
    pub const fn otp_disturbed0(&self) -> &OTP_DISTURBED0 {
        &self.otp_disturbed0
    }
    ///0x20 - BSEC_OTP_DISTURBED0 is used to report disturbed state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP). BSEC_OTP_DISTURBED1 is used to report disturbed state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_DISTURBED2 is used to report disturbed state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95.
    #[inline(always)]
    pub const fn otp_disturbed1(&self) -> &OTP_DISTURBED1 {
        &self.otp_disturbed1
    }
    ///0x24 - BSEC_OTP_DISTURBED0 is used to report disturbed state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP). BSEC_OTP_DISTURBED1 is used to report disturbed state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_DISTURBED2 is used to report disturbed state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95.
    #[inline(always)]
    pub const fn otp_disturbed2(&self) -> &OTP_DISTURBED2 {
        &self.otp_disturbed2
    }
    ///0x34 - BSEC_OTP_ERROR0 is used to report error state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP which are protected by 2:1 redundancy). BSEC_OTP_ERROR1 is used to report error state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 which are protected by 6-bit ECC. BSEC_OTP_ERROR2 is used to report error state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 which are protected by 6-bit ECC.
    #[inline(always)]
    pub const fn otp_error0(&self) -> &OTP_ERROR0 {
        &self.otp_error0
    }
    ///0x38 - BSEC_OTP_ERROR0 is used to report error state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP which are protected by 2:1 redundancy). BSEC_OTP_ERROR1 is used to report error state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 which are protected by 6-bit ECC. BSEC_OTP_ERROR2 is used to report error state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 which are protected by 6-bit ECC.
    #[inline(always)]
    pub const fn otp_error1(&self) -> &OTP_ERROR1 {
        &self.otp_error1
    }
    ///0x3c - BSEC_OTP_ERROR0 is used to report error state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP which are protected by 2:1 redundancy). BSEC_OTP_ERROR1 is used to report error state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 which are protected by 6-bit ECC. BSEC_OTP_ERROR2 is used to report error state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 which are protected by 6-bit ECC.
    #[inline(always)]
    pub const fn otp_error2(&self) -> &OTP_ERROR2 {
        &self.otp_error2
    }
    ///0x4c - BSEC_OTP_WLOCK0 is used to report permanent write lock of BSEC_OTP_DATA0 to BSEC_OTP_DATA31. BSEC_OTP_WLOCK1 is used to report permanent write lock of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_WLOCK2 is used to report permanent write lock of BSEC_OTP_DATA64 to BSEC_OTP_DATA95. Permanent write lock requires a programming sequence to lock a word (see section:Section3.3.6: OTP operations on page178).
    #[inline(always)]
    pub const fn otp_wrlock0(&self) -> &OTP_WRLOCK0 {
        &self.otp_wrlock0
    }
    ///0x50 - BSEC_OTP_WLOCK0 is used to report permanent write lock of BSEC_OTP_DATA0 to BSEC_OTP_DATA31. BSEC_OTP_WLOCK1 is used to report permanent write lock of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_WLOCK2 is used to report permanent write lock of BSEC_OTP_DATA64 to BSEC_OTP_DATA95. Permanent write lock requires a programming sequence to lock a word (see section:Section3.3.6: OTP operations on page178).
    #[inline(always)]
    pub const fn otp_wrlock1(&self) -> &OTP_WRLOCK1 {
        &self.otp_wrlock1
    }
    ///0x54 - BSEC_OTP_WLOCK0 is used to report permanent write lock of BSEC_OTP_DATA0 to BSEC_OTP_DATA31. BSEC_OTP_WLOCK1 is used to report permanent write lock of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_WLOCK2 is used to report permanent write lock of BSEC_OTP_DATA64 to BSEC_OTP_DATA95. Permanent write lock requires a programming sequence to lock a word (see section:Section3.3.6: OTP operations on page178).
    #[inline(always)]
    pub const fn otp_wrlock2(&self) -> &OTP_WRLOCK2 {
        &self.otp_wrlock2
    }
    ///0x64 - BSEC_OTP_SPLOCK0 is used to lock the programming of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset BSEC_OTP_SPLOCK1 is used to lock the programming of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset BSEC_OTP_SPLOCK2 is used to lock the programming of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset Attempt to sticky program locked OTP word are silently ignored.
    #[inline(always)]
    pub const fn otp_splock0(&self) -> &OTP_SPLOCK0 {
        &self.otp_splock0
    }
    ///0x68 - BSEC_OTP_SPLOCK0 is used to lock the programming of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset BSEC_OTP_SPLOCK1 is used to lock the programming of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset BSEC_OTP_SPLOCK2 is used to lock the programming of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset Attempt to sticky program locked OTP word are silently ignored.
    #[inline(always)]
    pub const fn otp_splock1(&self) -> &OTP_SPLOCK1 {
        &self.otp_splock1
    }
    ///0x6c - BSEC_OTP_SPLOCK0 is used to lock the programming of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset BSEC_OTP_SPLOCK1 is used to lock the programming of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset BSEC_OTP_SPLOCK2 is used to lock the programming of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset Attempt to sticky program locked OTP word are silently ignored.
    #[inline(always)]
    pub const fn otp_splock2(&self) -> &OTP_SPLOCK2 {
        &self.otp_splock2
    }
    ///0x7c - BSEC_OTP_SWLOCK0 is used to prevent writing to BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SWLOCK1 is used to prevent writing to BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SWLOCK2 is used to prevent writing to BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Write to shadow write locked BSEC_OTP_DATA word are silently ignored. Writing to OTP word 0 shadow is always prevented.
    #[inline(always)]
    pub const fn otp_swlock0(&self) -> &OTP_SWLOCK0 {
        &self.otp_swlock0
    }
    ///0x80 - BSEC_OTP_SWLOCK0 is used to prevent writing to BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SWLOCK1 is used to prevent writing to BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SWLOCK2 is used to prevent writing to BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Write to shadow write locked BSEC_OTP_DATA word are silently ignored. Writing to OTP word 0 shadow is always prevented.
    #[inline(always)]
    pub const fn otp_swlock1(&self) -> &OTP_SWLOCK1 {
        &self.otp_swlock1
    }
    ///0x84 - BSEC_OTP_SWLOCK0 is used to prevent writing to BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SWLOCK1 is used to prevent writing to BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SWLOCK2 is used to prevent writing to BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Write to shadow write locked BSEC_OTP_DATA word are silently ignored. Writing to OTP word 0 shadow is always prevented.
    #[inline(always)]
    pub const fn otp_swlock2(&self) -> &OTP_SWLOCK2 {
        &self.otp_swlock2
    }
    ///0x94 - BSEC_OTP_SRLOCK0 is used to prevent reloading of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SRLOCK1 is used to prevent reloading of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SRLOCK2 is used to prevent reloading of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Setting SRLOCK bits or attempt to reload a locked OTP do not clear the corresponding BSEC_OTP_DATAx shadow register. BSEC_OTP_SRLOCK0 bit 0 is controlled by hardware according to fuse_ok, writing to this bit has no effect.
    #[inline(always)]
    pub const fn otp_srlock0(&self) -> &OTP_SRLOCK0 {
        &self.otp_srlock0
    }
    ///0x98 - BSEC_OTP_SRLOCK0 is used to prevent reloading of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SRLOCK1 is used to prevent reloading of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SRLOCK2 is used to prevent reloading of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Setting SRLOCK bits or attempt to reload a locked OTP do not clear the corresponding BSEC_OTP_DATAx shadow register. BSEC_OTP_SRLOCK0 bit 0 is controlled by hardware according to fuse_ok, writing to this bit has no effect.
    #[inline(always)]
    pub const fn otp_srlock1(&self) -> &OTP_SRLOCK1 {
        &self.otp_srlock1
    }
    ///0x9c - BSEC_OTP_SRLOCK0 is used to prevent reloading of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SRLOCK1 is used to prevent reloading of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SRLOCK2 is used to prevent reloading of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Setting SRLOCK bits or attempt to reload a locked OTP do not clear the corresponding BSEC_OTP_DATAx shadow register. BSEC_OTP_SRLOCK0 bit 0 is controlled by hardware according to fuse_ok, writing to this bit has no effect.
    #[inline(always)]
    pub const fn otp_srlock2(&self) -> &OTP_SRLOCK2 {
        &self.otp_srlock2
    }
    ///0xac - BSEC JTAG input register
    #[inline(always)]
    pub const fn jtagin(&self) -> &JTAGIN {
        &self.jtagin
    }
    ///0xb0 - BSEC JTAG output register
    #[inline(always)]
    pub const fn jtagout(&self) -> &JTAGOUT {
        &self.jtagout
    }
    ///0xb4 - BSEC scratch register
    #[inline(always)]
    pub const fn scratch(&self) -> &SCRATCH {
        &self.scratch
    }
    ///0x200 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data0(&self) -> &OTP_DATA0 {
        &self.otp_data0
    }
    ///0x204 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data1(&self) -> &OTP_DATA1 {
        &self.otp_data1
    }
    ///0x208 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data2(&self) -> &OTP_DATA2 {
        &self.otp_data2
    }
    ///0x20c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data3(&self) -> &OTP_DATA3 {
        &self.otp_data3
    }
    ///0x210 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data4(&self) -> &OTP_DATA4 {
        &self.otp_data4
    }
    ///0x214 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data5(&self) -> &OTP_DATA5 {
        &self.otp_data5
    }
    ///0x218 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data6(&self) -> &OTP_DATA6 {
        &self.otp_data6
    }
    ///0x21c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data7(&self) -> &OTP_DATA7 {
        &self.otp_data7
    }
    ///0x220 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data8(&self) -> &OTP_DATA8 {
        &self.otp_data8
    }
    ///0x224 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data9(&self) -> &OTP_DATA9 {
        &self.otp_data9
    }
    ///0x228 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data10(&self) -> &OTP_DATA10 {
        &self.otp_data10
    }
    ///0x22c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data11(&self) -> &OTP_DATA11 {
        &self.otp_data11
    }
    ///0x230 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data12(&self) -> &OTP_DATA12 {
        &self.otp_data12
    }
    ///0x234 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data13(&self) -> &OTP_DATA13 {
        &self.otp_data13
    }
    ///0x238 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data14(&self) -> &OTP_DATA14 {
        &self.otp_data14
    }
    ///0x23c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data15(&self) -> &OTP_DATA15 {
        &self.otp_data15
    }
    ///0x240 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data16(&self) -> &OTP_DATA16 {
        &self.otp_data16
    }
    ///0x244 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data17(&self) -> &OTP_DATA17 {
        &self.otp_data17
    }
    ///0x248 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data18(&self) -> &OTP_DATA18 {
        &self.otp_data18
    }
    ///0x24c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data19(&self) -> &OTP_DATA19 {
        &self.otp_data19
    }
    ///0x250 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data20(&self) -> &OTP_DATA20 {
        &self.otp_data20
    }
    ///0x254 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data21(&self) -> &OTP_DATA21 {
        &self.otp_data21
    }
    ///0x258 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data22(&self) -> &OTP_DATA22 {
        &self.otp_data22
    }
    ///0x25c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data23(&self) -> &OTP_DATA23 {
        &self.otp_data23
    }
    ///0x260 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data24(&self) -> &OTP_DATA24 {
        &self.otp_data24
    }
    ///0x264 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data25(&self) -> &OTP_DATA25 {
        &self.otp_data25
    }
    ///0x268 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data26(&self) -> &OTP_DATA26 {
        &self.otp_data26
    }
    ///0x26c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data27(&self) -> &OTP_DATA27 {
        &self.otp_data27
    }
    ///0x270 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data28(&self) -> &OTP_DATA28 {
        &self.otp_data28
    }
    ///0x274 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data29(&self) -> &OTP_DATA29 {
        &self.otp_data29
    }
    ///0x278 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data30(&self) -> &OTP_DATA30 {
        &self.otp_data30
    }
    ///0x27c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data31(&self) -> &OTP_DATA31 {
        &self.otp_data31
    }
    ///0x280 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data32(&self) -> &OTP_DATA32 {
        &self.otp_data32
    }
    ///0x284 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data33(&self) -> &OTP_DATA33 {
        &self.otp_data33
    }
    ///0x288 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data34(&self) -> &OTP_DATA34 {
        &self.otp_data34
    }
    ///0x28c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data35(&self) -> &OTP_DATA35 {
        &self.otp_data35
    }
    ///0x290 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data36(&self) -> &OTP_DATA36 {
        &self.otp_data36
    }
    ///0x294 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data37(&self) -> &OTP_DATA37 {
        &self.otp_data37
    }
    ///0x298 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data38(&self) -> &OTP_DATA38 {
        &self.otp_data38
    }
    ///0x29c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data39(&self) -> &OTP_DATA39 {
        &self.otp_data39
    }
    ///0x2a0 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data40(&self) -> &OTP_DATA40 {
        &self.otp_data40
    }
    ///0x2a4 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data41(&self) -> &OTP_DATA41 {
        &self.otp_data41
    }
    ///0x2a8 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data42(&self) -> &OTP_DATA42 {
        &self.otp_data42
    }
    ///0x2ac - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data43(&self) -> &OTP_DATA43 {
        &self.otp_data43
    }
    ///0x2b0 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data44(&self) -> &OTP_DATA44 {
        &self.otp_data44
    }
    ///0x2b4 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data45(&self) -> &OTP_DATA45 {
        &self.otp_data45
    }
    ///0x2b8 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data46(&self) -> &OTP_DATA46 {
        &self.otp_data46
    }
    ///0x2bc - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data47(&self) -> &OTP_DATA47 {
        &self.otp_data47
    }
    ///0x2c0 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data48(&self) -> &OTP_DATA48 {
        &self.otp_data48
    }
    ///0x2c4 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data49(&self) -> &OTP_DATA49 {
        &self.otp_data49
    }
    ///0x2c8 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data50(&self) -> &OTP_DATA50 {
        &self.otp_data50
    }
    ///0x2cc - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data51(&self) -> &OTP_DATA51 {
        &self.otp_data51
    }
    ///0x2d0 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data52(&self) -> &OTP_DATA52 {
        &self.otp_data52
    }
    ///0x2d4 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data53(&self) -> &OTP_DATA53 {
        &self.otp_data53
    }
    ///0x2d8 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data54(&self) -> &OTP_DATA54 {
        &self.otp_data54
    }
    ///0x2dc - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data55(&self) -> &OTP_DATA55 {
        &self.otp_data55
    }
    ///0x2e0 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data56(&self) -> &OTP_DATA56 {
        &self.otp_data56
    }
    ///0x2e4 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data57(&self) -> &OTP_DATA57 {
        &self.otp_data57
    }
    ///0x2e8 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data58(&self) -> &OTP_DATA58 {
        &self.otp_data58
    }
    ///0x2ec - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data59(&self) -> &OTP_DATA59 {
        &self.otp_data59
    }
    ///0x2f0 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data60(&self) -> &OTP_DATA60 {
        &self.otp_data60
    }
    ///0x2f4 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data61(&self) -> &OTP_DATA61 {
        &self.otp_data61
    }
    ///0x2f8 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data62(&self) -> &OTP_DATA62 {
        &self.otp_data62
    }
    ///0x2fc - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data63(&self) -> &OTP_DATA63 {
        &self.otp_data63
    }
    ///0x300 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data64(&self) -> &OTP_DATA64 {
        &self.otp_data64
    }
    ///0x304 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data65(&self) -> &OTP_DATA65 {
        &self.otp_data65
    }
    ///0x308 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data66(&self) -> &OTP_DATA66 {
        &self.otp_data66
    }
    ///0x30c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data67(&self) -> &OTP_DATA67 {
        &self.otp_data67
    }
    ///0x310 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data68(&self) -> &OTP_DATA68 {
        &self.otp_data68
    }
    ///0x314 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data69(&self) -> &OTP_DATA69 {
        &self.otp_data69
    }
    ///0x318 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data70(&self) -> &OTP_DATA70 {
        &self.otp_data70
    }
    ///0x31c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data71(&self) -> &OTP_DATA71 {
        &self.otp_data71
    }
    ///0x320 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data72(&self) -> &OTP_DATA72 {
        &self.otp_data72
    }
    ///0x324 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data73(&self) -> &OTP_DATA73 {
        &self.otp_data73
    }
    ///0x328 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data74(&self) -> &OTP_DATA74 {
        &self.otp_data74
    }
    ///0x32c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data75(&self) -> &OTP_DATA75 {
        &self.otp_data75
    }
    ///0x330 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data76(&self) -> &OTP_DATA76 {
        &self.otp_data76
    }
    ///0x334 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data77(&self) -> &OTP_DATA77 {
        &self.otp_data77
    }
    ///0x338 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data78(&self) -> &OTP_DATA78 {
        &self.otp_data78
    }
    ///0x33c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data79(&self) -> &OTP_DATA79 {
        &self.otp_data79
    }
    ///0x340 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data80(&self) -> &OTP_DATA80 {
        &self.otp_data80
    }
    ///0x344 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data81(&self) -> &OTP_DATA81 {
        &self.otp_data81
    }
    ///0x348 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data82(&self) -> &OTP_DATA82 {
        &self.otp_data82
    }
    ///0x34c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data83(&self) -> &OTP_DATA83 {
        &self.otp_data83
    }
    ///0x350 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data84(&self) -> &OTP_DATA84 {
        &self.otp_data84
    }
    ///0x354 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data85(&self) -> &OTP_DATA85 {
        &self.otp_data85
    }
    ///0x358 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data86(&self) -> &OTP_DATA86 {
        &self.otp_data86
    }
    ///0x35c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data87(&self) -> &OTP_DATA87 {
        &self.otp_data87
    }
    ///0x360 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data88(&self) -> &OTP_DATA88 {
        &self.otp_data88
    }
    ///0x364 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data89(&self) -> &OTP_DATA89 {
        &self.otp_data89
    }
    ///0x368 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data90(&self) -> &OTP_DATA90 {
        &self.otp_data90
    }
    ///0x36c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data91(&self) -> &OTP_DATA91 {
        &self.otp_data91
    }
    ///0x370 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data92(&self) -> &OTP_DATA92 {
        &self.otp_data92
    }
    ///0x374 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data93(&self) -> &OTP_DATA93 {
        &self.otp_data93
    }
    ///0x378 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data94(&self) -> &OTP_DATA94 {
        &self.otp_data94
    }
    ///0x37c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    #[inline(always)]
    pub const fn otp_data95(&self) -> &OTP_DATA95 {
        &self.otp_data95
    }
    ///0xff0 - BSEC hardware configuration register
    #[inline(always)]
    pub const fn hwcfgr(&self) -> &HWCFGR {
        &self.hwcfgr
    }
    ///0xff4 - BSEC version register
    #[inline(always)]
    pub const fn verr(&self) -> &VERR {
        &self.verr
    }
    ///0xff8 - BSEC identification register
    #[inline(always)]
    pub const fn ipidr(&self) -> &IPIDR {
        &self.ipidr
    }
    ///0xffc - BSEC size identification register
    #[inline(always)]
    pub const fn sidr(&self) -> &SIDR {
        &self.sidr
    }
}
/**OTP_CONFIG (rw) register accessor: BSEC OTP configuration register

You can [`read`](crate::Reg::read) this register and get [`otp_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_CONFIG)

For information about available fields see [`mod@otp_config`] module*/
pub type OTP_CONFIG = crate::Reg<otp_config::OTP_CONFIGrs>;
///BSEC OTP configuration register
pub mod otp_config;
/**OTP_CONTROL (rw) register accessor: BSEC OTP control register

You can [`read`](crate::Reg::read) this register and get [`otp_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_CONTROL)

For information about available fields see [`mod@otp_control`] module*/
pub type OTP_CONTROL = crate::Reg<otp_control::OTP_CONTROLrs>;
///BSEC OTP control register
pub mod otp_control;
/**OTP_WRDATA (rw) register accessor: BSEC OTP write data register

You can [`read`](crate::Reg::read) this register and get [`otp_wrdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_wrdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_WRDATA)

For information about available fields see [`mod@otp_wrdata`] module*/
pub type OTP_WRDATA = crate::Reg<otp_wrdata::OTP_WRDATArs>;
///BSEC OTP write data register
pub mod otp_wrdata;
/**OTP_STATUS (r) register accessor: BSEC OTP status register

You can [`read`](crate::Reg::read) this register and get [`otp_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_STATUS)

For information about available fields see [`mod@otp_status`] module*/
pub type OTP_STATUS = crate::Reg<otp_status::OTP_STATUSrs>;
///BSEC OTP status register
pub mod otp_status;
/**OTP_LOCK (rw) register accessor: BSEC OTP lock configuration register

You can [`read`](crate::Reg::read) this register and get [`otp_lock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_lock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_LOCK)

For information about available fields see [`mod@otp_lock`] module*/
pub type OTP_LOCK = crate::Reg<otp_lock::OTP_LOCKrs>;
///BSEC OTP lock configuration register
pub mod otp_lock;
/**DENABLE (rw) register accessor: reset value depends on OTP secure mode according toTable18: BSEC_DENABLE default values after reset on page181.

You can [`read`](crate::Reg::read) this register and get [`denable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`denable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:DENABLE)

For information about available fields see [`mod@denable`] module*/
pub type DENABLE = crate::Reg<denable::DENABLErs>;
///reset value depends on OTP secure mode according toTable18: BSEC_DENABLE default values after reset on page181.
pub mod denable;
/**OTP_DISTURBED0 (r) register accessor: BSEC_OTP_DISTURBED0 is used to report disturbed state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP). BSEC_OTP_DISTURBED1 is used to report disturbed state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_DISTURBED2 is used to report disturbed state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95.

You can [`read`](crate::Reg::read) this register and get [`otp_disturbed0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DISTURBED0)

For information about available fields see [`mod@otp_disturbed0`] module*/
pub type OTP_DISTURBED0 = crate::Reg<otp_disturbed0::OTP_DISTURBED0rs>;
///BSEC_OTP_DISTURBED0 is used to report disturbed state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP). BSEC_OTP_DISTURBED1 is used to report disturbed state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_DISTURBED2 is used to report disturbed state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95.
pub mod otp_disturbed0;
/**OTP_DISTURBED1 (r) register accessor: BSEC_OTP_DISTURBED0 is used to report disturbed state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP). BSEC_OTP_DISTURBED1 is used to report disturbed state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_DISTURBED2 is used to report disturbed state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95.

You can [`read`](crate::Reg::read) this register and get [`otp_disturbed1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DISTURBED1)

For information about available fields see [`mod@otp_disturbed1`] module*/
pub type OTP_DISTURBED1 = crate::Reg<otp_disturbed1::OTP_DISTURBED1rs>;
///BSEC_OTP_DISTURBED0 is used to report disturbed state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP). BSEC_OTP_DISTURBED1 is used to report disturbed state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_DISTURBED2 is used to report disturbed state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95.
pub mod otp_disturbed1;
/**OTP_DISTURBED2 (r) register accessor: BSEC_OTP_DISTURBED0 is used to report disturbed state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP). BSEC_OTP_DISTURBED1 is used to report disturbed state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_DISTURBED2 is used to report disturbed state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95.

You can [`read`](crate::Reg::read) this register and get [`otp_disturbed2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DISTURBED2)

For information about available fields see [`mod@otp_disturbed2`] module*/
pub type OTP_DISTURBED2 = crate::Reg<otp_disturbed2::OTP_DISTURBED2rs>;
///BSEC_OTP_DISTURBED0 is used to report disturbed state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP). BSEC_OTP_DISTURBED1 is used to report disturbed state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_DISTURBED2 is used to report disturbed state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95.
pub mod otp_disturbed2;
/**OTP_ERROR0 (r) register accessor: BSEC_OTP_ERROR0 is used to report error state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP which are protected by 2:1 redundancy). BSEC_OTP_ERROR1 is used to report error state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 which are protected by 6-bit ECC. BSEC_OTP_ERROR2 is used to report error state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 which are protected by 6-bit ECC.

You can [`read`](crate::Reg::read) this register and get [`otp_error0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_ERROR0)

For information about available fields see [`mod@otp_error0`] module*/
pub type OTP_ERROR0 = crate::Reg<otp_error0::OTP_ERROR0rs>;
///BSEC_OTP_ERROR0 is used to report error state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP which are protected by 2:1 redundancy). BSEC_OTP_ERROR1 is used to report error state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 which are protected by 6-bit ECC. BSEC_OTP_ERROR2 is used to report error state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 which are protected by 6-bit ECC.
pub mod otp_error0;
/**OTP_ERROR1 (r) register accessor: BSEC_OTP_ERROR0 is used to report error state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP which are protected by 2:1 redundancy). BSEC_OTP_ERROR1 is used to report error state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 which are protected by 6-bit ECC. BSEC_OTP_ERROR2 is used to report error state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 which are protected by 6-bit ECC.

You can [`read`](crate::Reg::read) this register and get [`otp_error1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_ERROR1)

For information about available fields see [`mod@otp_error1`] module*/
pub type OTP_ERROR1 = crate::Reg<otp_error1::OTP_ERROR1rs>;
///BSEC_OTP_ERROR0 is used to report error state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP which are protected by 2:1 redundancy). BSEC_OTP_ERROR1 is used to report error state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 which are protected by 6-bit ECC. BSEC_OTP_ERROR2 is used to report error state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 which are protected by 6-bit ECC.
pub mod otp_error1;
/**OTP_ERROR2 (r) register accessor: BSEC_OTP_ERROR0 is used to report error state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP which are protected by 2:1 redundancy). BSEC_OTP_ERROR1 is used to report error state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 which are protected by 6-bit ECC. BSEC_OTP_ERROR2 is used to report error state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 which are protected by 6-bit ECC.

You can [`read`](crate::Reg::read) this register and get [`otp_error2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_ERROR2)

For information about available fields see [`mod@otp_error2`] module*/
pub type OTP_ERROR2 = crate::Reg<otp_error2::OTP_ERROR2rs>;
///BSEC_OTP_ERROR0 is used to report error state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP which are protected by 2:1 redundancy). BSEC_OTP_ERROR1 is used to report error state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 which are protected by 6-bit ECC. BSEC_OTP_ERROR2 is used to report error state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 which are protected by 6-bit ECC.
pub mod otp_error2;
/**OTP_WRLOCK0 (r) register accessor: BSEC_OTP_WLOCK0 is used to report permanent write lock of BSEC_OTP_DATA0 to BSEC_OTP_DATA31. BSEC_OTP_WLOCK1 is used to report permanent write lock of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_WLOCK2 is used to report permanent write lock of BSEC_OTP_DATA64 to BSEC_OTP_DATA95. Permanent write lock requires a programming sequence to lock a word (see section:Section3.3.6: OTP operations on page178).

You can [`read`](crate::Reg::read) this register and get [`otp_wrlock0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_WRLOCK0)

For information about available fields see [`mod@otp_wrlock0`] module*/
pub type OTP_WRLOCK0 = crate::Reg<otp_wrlock0::OTP_WRLOCK0rs>;
///BSEC_OTP_WLOCK0 is used to report permanent write lock of BSEC_OTP_DATA0 to BSEC_OTP_DATA31. BSEC_OTP_WLOCK1 is used to report permanent write lock of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_WLOCK2 is used to report permanent write lock of BSEC_OTP_DATA64 to BSEC_OTP_DATA95. Permanent write lock requires a programming sequence to lock a word (see section:Section3.3.6: OTP operations on page178).
pub mod otp_wrlock0;
/**OTP_WRLOCK1 (r) register accessor: BSEC_OTP_WLOCK0 is used to report permanent write lock of BSEC_OTP_DATA0 to BSEC_OTP_DATA31. BSEC_OTP_WLOCK1 is used to report permanent write lock of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_WLOCK2 is used to report permanent write lock of BSEC_OTP_DATA64 to BSEC_OTP_DATA95. Permanent write lock requires a programming sequence to lock a word (see section:Section3.3.6: OTP operations on page178).

You can [`read`](crate::Reg::read) this register and get [`otp_wrlock1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_WRLOCK1)

For information about available fields see [`mod@otp_wrlock1`] module*/
pub type OTP_WRLOCK1 = crate::Reg<otp_wrlock1::OTP_WRLOCK1rs>;
///BSEC_OTP_WLOCK0 is used to report permanent write lock of BSEC_OTP_DATA0 to BSEC_OTP_DATA31. BSEC_OTP_WLOCK1 is used to report permanent write lock of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_WLOCK2 is used to report permanent write lock of BSEC_OTP_DATA64 to BSEC_OTP_DATA95. Permanent write lock requires a programming sequence to lock a word (see section:Section3.3.6: OTP operations on page178).
pub mod otp_wrlock1;
/**OTP_WRLOCK2 (r) register accessor: BSEC_OTP_WLOCK0 is used to report permanent write lock of BSEC_OTP_DATA0 to BSEC_OTP_DATA31. BSEC_OTP_WLOCK1 is used to report permanent write lock of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_WLOCK2 is used to report permanent write lock of BSEC_OTP_DATA64 to BSEC_OTP_DATA95. Permanent write lock requires a programming sequence to lock a word (see section:Section3.3.6: OTP operations on page178).

You can [`read`](crate::Reg::read) this register and get [`otp_wrlock2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_WRLOCK2)

For information about available fields see [`mod@otp_wrlock2`] module*/
pub type OTP_WRLOCK2 = crate::Reg<otp_wrlock2::OTP_WRLOCK2rs>;
///BSEC_OTP_WLOCK0 is used to report permanent write lock of BSEC_OTP_DATA0 to BSEC_OTP_DATA31. BSEC_OTP_WLOCK1 is used to report permanent write lock of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_WLOCK2 is used to report permanent write lock of BSEC_OTP_DATA64 to BSEC_OTP_DATA95. Permanent write lock requires a programming sequence to lock a word (see section:Section3.3.6: OTP operations on page178).
pub mod otp_wrlock2;
/**OTP_SPLOCK0 (rw) register accessor: BSEC_OTP_SPLOCK0 is used to lock the programming of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset BSEC_OTP_SPLOCK1 is used to lock the programming of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset BSEC_OTP_SPLOCK2 is used to lock the programming of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset Attempt to sticky program locked OTP word are silently ignored.

You can [`read`](crate::Reg::read) this register and get [`otp_splock0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_splock0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_SPLOCK0)

For information about available fields see [`mod@otp_splock0`] module*/
pub type OTP_SPLOCK0 = crate::Reg<otp_splock0::OTP_SPLOCK0rs>;
///BSEC_OTP_SPLOCK0 is used to lock the programming of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset BSEC_OTP_SPLOCK1 is used to lock the programming of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset BSEC_OTP_SPLOCK2 is used to lock the programming of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset Attempt to sticky program locked OTP word are silently ignored.
pub mod otp_splock0;
/**OTP_SPLOCK1 (rw) register accessor: BSEC_OTP_SPLOCK0 is used to lock the programming of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset BSEC_OTP_SPLOCK1 is used to lock the programming of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset BSEC_OTP_SPLOCK2 is used to lock the programming of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset Attempt to sticky program locked OTP word are silently ignored.

You can [`read`](crate::Reg::read) this register and get [`otp_splock1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_splock1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_SPLOCK1)

For information about available fields see [`mod@otp_splock1`] module*/
pub type OTP_SPLOCK1 = crate::Reg<otp_splock1::OTP_SPLOCK1rs>;
///BSEC_OTP_SPLOCK0 is used to lock the programming of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset BSEC_OTP_SPLOCK1 is used to lock the programming of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset BSEC_OTP_SPLOCK2 is used to lock the programming of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset Attempt to sticky program locked OTP word are silently ignored.
pub mod otp_splock1;
/**OTP_SPLOCK2 (rw) register accessor: BSEC_OTP_SPLOCK0 is used to lock the programming of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset BSEC_OTP_SPLOCK1 is used to lock the programming of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset BSEC_OTP_SPLOCK2 is used to lock the programming of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset Attempt to sticky program locked OTP word are silently ignored.

You can [`read`](crate::Reg::read) this register and get [`otp_splock2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_splock2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_SPLOCK2)

For information about available fields see [`mod@otp_splock2`] module*/
pub type OTP_SPLOCK2 = crate::Reg<otp_splock2::OTP_SPLOCK2rs>;
///BSEC_OTP_SPLOCK0 is used to lock the programming of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset BSEC_OTP_SPLOCK1 is used to lock the programming of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset BSEC_OTP_SPLOCK2 is used to lock the programming of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset Attempt to sticky program locked OTP word are silently ignored.
pub mod otp_splock2;
/**OTP_SWLOCK0 (rw) register accessor: BSEC_OTP_SWLOCK0 is used to prevent writing to BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SWLOCK1 is used to prevent writing to BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SWLOCK2 is used to prevent writing to BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Write to shadow write locked BSEC_OTP_DATA word are silently ignored. Writing to OTP word 0 shadow is always prevented.

You can [`read`](crate::Reg::read) this register and get [`otp_swlock0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_swlock0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_SWLOCK0)

For information about available fields see [`mod@otp_swlock0`] module*/
pub type OTP_SWLOCK0 = crate::Reg<otp_swlock0::OTP_SWLOCK0rs>;
///BSEC_OTP_SWLOCK0 is used to prevent writing to BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SWLOCK1 is used to prevent writing to BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SWLOCK2 is used to prevent writing to BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Write to shadow write locked BSEC_OTP_DATA word are silently ignored. Writing to OTP word 0 shadow is always prevented.
pub mod otp_swlock0;
/**OTP_SWLOCK1 (rw) register accessor: BSEC_OTP_SWLOCK0 is used to prevent writing to BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SWLOCK1 is used to prevent writing to BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SWLOCK2 is used to prevent writing to BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Write to shadow write locked BSEC_OTP_DATA word are silently ignored. Writing to OTP word 0 shadow is always prevented.

You can [`read`](crate::Reg::read) this register and get [`otp_swlock1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_swlock1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_SWLOCK1)

For information about available fields see [`mod@otp_swlock1`] module*/
pub type OTP_SWLOCK1 = crate::Reg<otp_swlock1::OTP_SWLOCK1rs>;
///BSEC_OTP_SWLOCK0 is used to prevent writing to BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SWLOCK1 is used to prevent writing to BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SWLOCK2 is used to prevent writing to BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Write to shadow write locked BSEC_OTP_DATA word are silently ignored. Writing to OTP word 0 shadow is always prevented.
pub mod otp_swlock1;
/**OTP_SWLOCK2 (rw) register accessor: BSEC_OTP_SWLOCK0 is used to prevent writing to BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SWLOCK1 is used to prevent writing to BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SWLOCK2 is used to prevent writing to BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Write to shadow write locked BSEC_OTP_DATA word are silently ignored. Writing to OTP word 0 shadow is always prevented.

You can [`read`](crate::Reg::read) this register and get [`otp_swlock2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_swlock2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_SWLOCK2)

For information about available fields see [`mod@otp_swlock2`] module*/
pub type OTP_SWLOCK2 = crate::Reg<otp_swlock2::OTP_SWLOCK2rs>;
///BSEC_OTP_SWLOCK0 is used to prevent writing to BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SWLOCK1 is used to prevent writing to BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SWLOCK2 is used to prevent writing to BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Write to shadow write locked BSEC_OTP_DATA word are silently ignored. Writing to OTP word 0 shadow is always prevented.
pub mod otp_swlock2;
/**OTP_SRLOCK0 (rw) register accessor: BSEC_OTP_SRLOCK0 is used to prevent reloading of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SRLOCK1 is used to prevent reloading of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SRLOCK2 is used to prevent reloading of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Setting SRLOCK bits or attempt to reload a locked OTP do not clear the corresponding BSEC_OTP_DATAx shadow register. BSEC_OTP_SRLOCK0 bit 0 is controlled by hardware according to fuse_ok, writing to this bit has no effect.

You can [`read`](crate::Reg::read) this register and get [`otp_srlock0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_srlock0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_SRLOCK0)

For information about available fields see [`mod@otp_srlock0`] module*/
pub type OTP_SRLOCK0 = crate::Reg<otp_srlock0::OTP_SRLOCK0rs>;
///BSEC_OTP_SRLOCK0 is used to prevent reloading of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SRLOCK1 is used to prevent reloading of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SRLOCK2 is used to prevent reloading of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Setting SRLOCK bits or attempt to reload a locked OTP do not clear the corresponding BSEC_OTP_DATAx shadow register. BSEC_OTP_SRLOCK0 bit 0 is controlled by hardware according to fuse_ok, writing to this bit has no effect.
pub mod otp_srlock0;
/**OTP_SRLOCK1 (rw) register accessor: BSEC_OTP_SRLOCK0 is used to prevent reloading of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SRLOCK1 is used to prevent reloading of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SRLOCK2 is used to prevent reloading of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Setting SRLOCK bits or attempt to reload a locked OTP do not clear the corresponding BSEC_OTP_DATAx shadow register. BSEC_OTP_SRLOCK0 bit 0 is controlled by hardware according to fuse_ok, writing to this bit has no effect.

You can [`read`](crate::Reg::read) this register and get [`otp_srlock1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_srlock1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_SRLOCK1)

For information about available fields see [`mod@otp_srlock1`] module*/
pub type OTP_SRLOCK1 = crate::Reg<otp_srlock1::OTP_SRLOCK1rs>;
///BSEC_OTP_SRLOCK0 is used to prevent reloading of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SRLOCK1 is used to prevent reloading of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SRLOCK2 is used to prevent reloading of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Setting SRLOCK bits or attempt to reload a locked OTP do not clear the corresponding BSEC_OTP_DATAx shadow register. BSEC_OTP_SRLOCK0 bit 0 is controlled by hardware according to fuse_ok, writing to this bit has no effect.
pub mod otp_srlock1;
/**OTP_SRLOCK2 (rw) register accessor: BSEC_OTP_SRLOCK0 is used to prevent reloading of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SRLOCK1 is used to prevent reloading of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SRLOCK2 is used to prevent reloading of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Setting SRLOCK bits or attempt to reload a locked OTP do not clear the corresponding BSEC_OTP_DATAx shadow register. BSEC_OTP_SRLOCK0 bit 0 is controlled by hardware according to fuse_ok, writing to this bit has no effect.

You can [`read`](crate::Reg::read) this register and get [`otp_srlock2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_srlock2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_SRLOCK2)

For information about available fields see [`mod@otp_srlock2`] module*/
pub type OTP_SRLOCK2 = crate::Reg<otp_srlock2::OTP_SRLOCK2rs>;
///BSEC_OTP_SRLOCK0 is used to prevent reloading of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SRLOCK1 is used to prevent reloading of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SRLOCK2 is used to prevent reloading of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Setting SRLOCK bits or attempt to reload a locked OTP do not clear the corresponding BSEC_OTP_DATAx shadow register. BSEC_OTP_SRLOCK0 bit 0 is controlled by hardware according to fuse_ok, writing to this bit has no effect.
pub mod otp_srlock2;
/**JTAGIN (r) register accessor: BSEC JTAG input register

You can [`read`](crate::Reg::read) this register and get [`jtagin::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:JTAGIN)

For information about available fields see [`mod@jtagin`] module*/
pub type JTAGIN = crate::Reg<jtagin::JTAGINrs>;
///BSEC JTAG input register
pub mod jtagin;
/**JTAGOUT (rw) register accessor: BSEC JTAG output register

You can [`read`](crate::Reg::read) this register and get [`jtagout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jtagout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:JTAGOUT)

For information about available fields see [`mod@jtagout`] module*/
pub type JTAGOUT = crate::Reg<jtagout::JTAGOUTrs>;
///BSEC JTAG output register
pub mod jtagout;
/**SCRATCH (rw) register accessor: BSEC scratch register

You can [`read`](crate::Reg::read) this register and get [`scratch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scratch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:SCRATCH)

For information about available fields see [`mod@scratch`] module*/
pub type SCRATCH = crate::Reg<scratch::SCRATCHrs>;
///BSEC scratch register
pub mod scratch;
/**OTP_DATA0 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA0)

For information about available fields see [`mod@otp_data0`] module*/
pub type OTP_DATA0 = crate::Reg<otp_data0::OTP_DATA0rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data0;
/**OTP_DATA1 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA1)

For information about available fields see [`mod@otp_data1`] module*/
pub type OTP_DATA1 = crate::Reg<otp_data1::OTP_DATA1rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data1;
/**OTP_DATA2 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA2)

For information about available fields see [`mod@otp_data2`] module*/
pub type OTP_DATA2 = crate::Reg<otp_data2::OTP_DATA2rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data2;
/**OTP_DATA3 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA3)

For information about available fields see [`mod@otp_data3`] module*/
pub type OTP_DATA3 = crate::Reg<otp_data3::OTP_DATA3rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data3;
/**OTP_DATA4 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA4)

For information about available fields see [`mod@otp_data4`] module*/
pub type OTP_DATA4 = crate::Reg<otp_data4::OTP_DATA4rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data4;
/**OTP_DATA5 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA5)

For information about available fields see [`mod@otp_data5`] module*/
pub type OTP_DATA5 = crate::Reg<otp_data5::OTP_DATA5rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data5;
/**OTP_DATA6 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA6)

For information about available fields see [`mod@otp_data6`] module*/
pub type OTP_DATA6 = crate::Reg<otp_data6::OTP_DATA6rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data6;
/**OTP_DATA7 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA7)

For information about available fields see [`mod@otp_data7`] module*/
pub type OTP_DATA7 = crate::Reg<otp_data7::OTP_DATA7rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data7;
/**OTP_DATA8 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA8)

For information about available fields see [`mod@otp_data8`] module*/
pub type OTP_DATA8 = crate::Reg<otp_data8::OTP_DATA8rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data8;
/**OTP_DATA9 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA9)

For information about available fields see [`mod@otp_data9`] module*/
pub type OTP_DATA9 = crate::Reg<otp_data9::OTP_DATA9rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data9;
/**OTP_DATA10 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA10)

For information about available fields see [`mod@otp_data10`] module*/
pub type OTP_DATA10 = crate::Reg<otp_data10::OTP_DATA10rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data10;
/**OTP_DATA11 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA11)

For information about available fields see [`mod@otp_data11`] module*/
pub type OTP_DATA11 = crate::Reg<otp_data11::OTP_DATA11rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data11;
/**OTP_DATA12 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA12)

For information about available fields see [`mod@otp_data12`] module*/
pub type OTP_DATA12 = crate::Reg<otp_data12::OTP_DATA12rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data12;
/**OTP_DATA13 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA13)

For information about available fields see [`mod@otp_data13`] module*/
pub type OTP_DATA13 = crate::Reg<otp_data13::OTP_DATA13rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data13;
/**OTP_DATA14 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA14)

For information about available fields see [`mod@otp_data14`] module*/
pub type OTP_DATA14 = crate::Reg<otp_data14::OTP_DATA14rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data14;
/**OTP_DATA15 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA15)

For information about available fields see [`mod@otp_data15`] module*/
pub type OTP_DATA15 = crate::Reg<otp_data15::OTP_DATA15rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data15;
/**OTP_DATA16 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA16)

For information about available fields see [`mod@otp_data16`] module*/
pub type OTP_DATA16 = crate::Reg<otp_data16::OTP_DATA16rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data16;
/**OTP_DATA17 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA17)

For information about available fields see [`mod@otp_data17`] module*/
pub type OTP_DATA17 = crate::Reg<otp_data17::OTP_DATA17rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data17;
/**OTP_DATA18 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA18)

For information about available fields see [`mod@otp_data18`] module*/
pub type OTP_DATA18 = crate::Reg<otp_data18::OTP_DATA18rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data18;
/**OTP_DATA19 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA19)

For information about available fields see [`mod@otp_data19`] module*/
pub type OTP_DATA19 = crate::Reg<otp_data19::OTP_DATA19rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data19;
/**OTP_DATA20 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA20)

For information about available fields see [`mod@otp_data20`] module*/
pub type OTP_DATA20 = crate::Reg<otp_data20::OTP_DATA20rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data20;
/**OTP_DATA21 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA21)

For information about available fields see [`mod@otp_data21`] module*/
pub type OTP_DATA21 = crate::Reg<otp_data21::OTP_DATA21rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data21;
/**OTP_DATA22 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA22)

For information about available fields see [`mod@otp_data22`] module*/
pub type OTP_DATA22 = crate::Reg<otp_data22::OTP_DATA22rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data22;
/**OTP_DATA23 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA23)

For information about available fields see [`mod@otp_data23`] module*/
pub type OTP_DATA23 = crate::Reg<otp_data23::OTP_DATA23rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data23;
/**OTP_DATA24 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA24)

For information about available fields see [`mod@otp_data24`] module*/
pub type OTP_DATA24 = crate::Reg<otp_data24::OTP_DATA24rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data24;
/**OTP_DATA25 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA25)

For information about available fields see [`mod@otp_data25`] module*/
pub type OTP_DATA25 = crate::Reg<otp_data25::OTP_DATA25rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data25;
/**OTP_DATA26 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA26)

For information about available fields see [`mod@otp_data26`] module*/
pub type OTP_DATA26 = crate::Reg<otp_data26::OTP_DATA26rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data26;
/**OTP_DATA27 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA27)

For information about available fields see [`mod@otp_data27`] module*/
pub type OTP_DATA27 = crate::Reg<otp_data27::OTP_DATA27rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data27;
/**OTP_DATA28 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA28)

For information about available fields see [`mod@otp_data28`] module*/
pub type OTP_DATA28 = crate::Reg<otp_data28::OTP_DATA28rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data28;
/**OTP_DATA29 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA29)

For information about available fields see [`mod@otp_data29`] module*/
pub type OTP_DATA29 = crate::Reg<otp_data29::OTP_DATA29rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data29;
/**OTP_DATA30 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA30)

For information about available fields see [`mod@otp_data30`] module*/
pub type OTP_DATA30 = crate::Reg<otp_data30::OTP_DATA30rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data30;
/**OTP_DATA31 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA31)

For information about available fields see [`mod@otp_data31`] module*/
pub type OTP_DATA31 = crate::Reg<otp_data31::OTP_DATA31rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data31;
/**OTP_DATA32 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data32::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data32::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA32)

For information about available fields see [`mod@otp_data32`] module*/
pub type OTP_DATA32 = crate::Reg<otp_data32::OTP_DATA32rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data32;
/**OTP_DATA33 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data33::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data33::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA33)

For information about available fields see [`mod@otp_data33`] module*/
pub type OTP_DATA33 = crate::Reg<otp_data33::OTP_DATA33rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data33;
/**OTP_DATA34 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data34::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data34::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA34)

For information about available fields see [`mod@otp_data34`] module*/
pub type OTP_DATA34 = crate::Reg<otp_data34::OTP_DATA34rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data34;
/**OTP_DATA35 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data35::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data35::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA35)

For information about available fields see [`mod@otp_data35`] module*/
pub type OTP_DATA35 = crate::Reg<otp_data35::OTP_DATA35rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data35;
/**OTP_DATA36 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data36::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data36::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA36)

For information about available fields see [`mod@otp_data36`] module*/
pub type OTP_DATA36 = crate::Reg<otp_data36::OTP_DATA36rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data36;
/**OTP_DATA37 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data37::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data37::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA37)

For information about available fields see [`mod@otp_data37`] module*/
pub type OTP_DATA37 = crate::Reg<otp_data37::OTP_DATA37rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data37;
/**OTP_DATA38 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data38::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data38::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA38)

For information about available fields see [`mod@otp_data38`] module*/
pub type OTP_DATA38 = crate::Reg<otp_data38::OTP_DATA38rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data38;
/**OTP_DATA39 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data39::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data39::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA39)

For information about available fields see [`mod@otp_data39`] module*/
pub type OTP_DATA39 = crate::Reg<otp_data39::OTP_DATA39rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data39;
/**OTP_DATA40 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data40::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data40::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA40)

For information about available fields see [`mod@otp_data40`] module*/
pub type OTP_DATA40 = crate::Reg<otp_data40::OTP_DATA40rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data40;
/**OTP_DATA41 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data41::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data41::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA41)

For information about available fields see [`mod@otp_data41`] module*/
pub type OTP_DATA41 = crate::Reg<otp_data41::OTP_DATA41rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data41;
/**OTP_DATA42 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data42::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data42::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA42)

For information about available fields see [`mod@otp_data42`] module*/
pub type OTP_DATA42 = crate::Reg<otp_data42::OTP_DATA42rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data42;
/**OTP_DATA43 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data43::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data43::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA43)

For information about available fields see [`mod@otp_data43`] module*/
pub type OTP_DATA43 = crate::Reg<otp_data43::OTP_DATA43rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data43;
/**OTP_DATA44 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data44::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data44::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA44)

For information about available fields see [`mod@otp_data44`] module*/
pub type OTP_DATA44 = crate::Reg<otp_data44::OTP_DATA44rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data44;
/**OTP_DATA45 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data45::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data45::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA45)

For information about available fields see [`mod@otp_data45`] module*/
pub type OTP_DATA45 = crate::Reg<otp_data45::OTP_DATA45rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data45;
/**OTP_DATA46 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data46::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data46::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA46)

For information about available fields see [`mod@otp_data46`] module*/
pub type OTP_DATA46 = crate::Reg<otp_data46::OTP_DATA46rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data46;
/**OTP_DATA47 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data47::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data47::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA47)

For information about available fields see [`mod@otp_data47`] module*/
pub type OTP_DATA47 = crate::Reg<otp_data47::OTP_DATA47rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data47;
/**OTP_DATA48 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data48::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data48::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA48)

For information about available fields see [`mod@otp_data48`] module*/
pub type OTP_DATA48 = crate::Reg<otp_data48::OTP_DATA48rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data48;
/**OTP_DATA49 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data49::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data49::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA49)

For information about available fields see [`mod@otp_data49`] module*/
pub type OTP_DATA49 = crate::Reg<otp_data49::OTP_DATA49rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data49;
/**OTP_DATA50 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data50::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data50::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA50)

For information about available fields see [`mod@otp_data50`] module*/
pub type OTP_DATA50 = crate::Reg<otp_data50::OTP_DATA50rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data50;
/**OTP_DATA51 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data51::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data51::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA51)

For information about available fields see [`mod@otp_data51`] module*/
pub type OTP_DATA51 = crate::Reg<otp_data51::OTP_DATA51rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data51;
/**OTP_DATA52 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data52::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data52::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA52)

For information about available fields see [`mod@otp_data52`] module*/
pub type OTP_DATA52 = crate::Reg<otp_data52::OTP_DATA52rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data52;
/**OTP_DATA53 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data53::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data53::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA53)

For information about available fields see [`mod@otp_data53`] module*/
pub type OTP_DATA53 = crate::Reg<otp_data53::OTP_DATA53rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data53;
/**OTP_DATA54 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data54::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data54::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA54)

For information about available fields see [`mod@otp_data54`] module*/
pub type OTP_DATA54 = crate::Reg<otp_data54::OTP_DATA54rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data54;
/**OTP_DATA55 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data55::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data55::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA55)

For information about available fields see [`mod@otp_data55`] module*/
pub type OTP_DATA55 = crate::Reg<otp_data55::OTP_DATA55rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data55;
/**OTP_DATA56 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data56::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data56::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA56)

For information about available fields see [`mod@otp_data56`] module*/
pub type OTP_DATA56 = crate::Reg<otp_data56::OTP_DATA56rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data56;
/**OTP_DATA57 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data57::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data57::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA57)

For information about available fields see [`mod@otp_data57`] module*/
pub type OTP_DATA57 = crate::Reg<otp_data57::OTP_DATA57rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data57;
/**OTP_DATA58 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data58::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data58::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA58)

For information about available fields see [`mod@otp_data58`] module*/
pub type OTP_DATA58 = crate::Reg<otp_data58::OTP_DATA58rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data58;
/**OTP_DATA59 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data59::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data59::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA59)

For information about available fields see [`mod@otp_data59`] module*/
pub type OTP_DATA59 = crate::Reg<otp_data59::OTP_DATA59rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data59;
/**OTP_DATA60 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data60::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data60::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA60)

For information about available fields see [`mod@otp_data60`] module*/
pub type OTP_DATA60 = crate::Reg<otp_data60::OTP_DATA60rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data60;
/**OTP_DATA61 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data61::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data61::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA61)

For information about available fields see [`mod@otp_data61`] module*/
pub type OTP_DATA61 = crate::Reg<otp_data61::OTP_DATA61rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data61;
/**OTP_DATA62 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data62::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data62::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA62)

For information about available fields see [`mod@otp_data62`] module*/
pub type OTP_DATA62 = crate::Reg<otp_data62::OTP_DATA62rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data62;
/**OTP_DATA63 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data63::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data63::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA63)

For information about available fields see [`mod@otp_data63`] module*/
pub type OTP_DATA63 = crate::Reg<otp_data63::OTP_DATA63rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data63;
/**OTP_DATA64 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data64::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data64::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA64)

For information about available fields see [`mod@otp_data64`] module*/
pub type OTP_DATA64 = crate::Reg<otp_data64::OTP_DATA64rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data64;
/**OTP_DATA65 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data65::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data65::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA65)

For information about available fields see [`mod@otp_data65`] module*/
pub type OTP_DATA65 = crate::Reg<otp_data65::OTP_DATA65rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data65;
/**OTP_DATA66 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data66::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data66::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA66)

For information about available fields see [`mod@otp_data66`] module*/
pub type OTP_DATA66 = crate::Reg<otp_data66::OTP_DATA66rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data66;
/**OTP_DATA67 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data67::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data67::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA67)

For information about available fields see [`mod@otp_data67`] module*/
pub type OTP_DATA67 = crate::Reg<otp_data67::OTP_DATA67rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data67;
/**OTP_DATA68 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data68::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data68::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA68)

For information about available fields see [`mod@otp_data68`] module*/
pub type OTP_DATA68 = crate::Reg<otp_data68::OTP_DATA68rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data68;
/**OTP_DATA69 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data69::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data69::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA69)

For information about available fields see [`mod@otp_data69`] module*/
pub type OTP_DATA69 = crate::Reg<otp_data69::OTP_DATA69rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data69;
/**OTP_DATA70 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data70::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data70::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA70)

For information about available fields see [`mod@otp_data70`] module*/
pub type OTP_DATA70 = crate::Reg<otp_data70::OTP_DATA70rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data70;
/**OTP_DATA71 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data71::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data71::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA71)

For information about available fields see [`mod@otp_data71`] module*/
pub type OTP_DATA71 = crate::Reg<otp_data71::OTP_DATA71rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data71;
/**OTP_DATA72 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data72::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data72::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA72)

For information about available fields see [`mod@otp_data72`] module*/
pub type OTP_DATA72 = crate::Reg<otp_data72::OTP_DATA72rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data72;
/**OTP_DATA73 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data73::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data73::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA73)

For information about available fields see [`mod@otp_data73`] module*/
pub type OTP_DATA73 = crate::Reg<otp_data73::OTP_DATA73rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data73;
/**OTP_DATA74 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data74::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data74::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA74)

For information about available fields see [`mod@otp_data74`] module*/
pub type OTP_DATA74 = crate::Reg<otp_data74::OTP_DATA74rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data74;
/**OTP_DATA75 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data75::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data75::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA75)

For information about available fields see [`mod@otp_data75`] module*/
pub type OTP_DATA75 = crate::Reg<otp_data75::OTP_DATA75rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data75;
/**OTP_DATA76 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data76::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data76::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA76)

For information about available fields see [`mod@otp_data76`] module*/
pub type OTP_DATA76 = crate::Reg<otp_data76::OTP_DATA76rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data76;
/**OTP_DATA77 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data77::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data77::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA77)

For information about available fields see [`mod@otp_data77`] module*/
pub type OTP_DATA77 = crate::Reg<otp_data77::OTP_DATA77rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data77;
/**OTP_DATA78 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data78::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data78::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA78)

For information about available fields see [`mod@otp_data78`] module*/
pub type OTP_DATA78 = crate::Reg<otp_data78::OTP_DATA78rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data78;
/**OTP_DATA79 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data79::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data79::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA79)

For information about available fields see [`mod@otp_data79`] module*/
pub type OTP_DATA79 = crate::Reg<otp_data79::OTP_DATA79rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data79;
/**OTP_DATA80 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data80::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data80::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA80)

For information about available fields see [`mod@otp_data80`] module*/
pub type OTP_DATA80 = crate::Reg<otp_data80::OTP_DATA80rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data80;
/**OTP_DATA81 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data81::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data81::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA81)

For information about available fields see [`mod@otp_data81`] module*/
pub type OTP_DATA81 = crate::Reg<otp_data81::OTP_DATA81rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data81;
/**OTP_DATA82 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data82::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data82::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA82)

For information about available fields see [`mod@otp_data82`] module*/
pub type OTP_DATA82 = crate::Reg<otp_data82::OTP_DATA82rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data82;
/**OTP_DATA83 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data83::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data83::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA83)

For information about available fields see [`mod@otp_data83`] module*/
pub type OTP_DATA83 = crate::Reg<otp_data83::OTP_DATA83rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data83;
/**OTP_DATA84 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data84::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data84::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA84)

For information about available fields see [`mod@otp_data84`] module*/
pub type OTP_DATA84 = crate::Reg<otp_data84::OTP_DATA84rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data84;
/**OTP_DATA85 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data85::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data85::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA85)

For information about available fields see [`mod@otp_data85`] module*/
pub type OTP_DATA85 = crate::Reg<otp_data85::OTP_DATA85rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data85;
/**OTP_DATA86 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data86::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data86::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA86)

For information about available fields see [`mod@otp_data86`] module*/
pub type OTP_DATA86 = crate::Reg<otp_data86::OTP_DATA86rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data86;
/**OTP_DATA87 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data87::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data87::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA87)

For information about available fields see [`mod@otp_data87`] module*/
pub type OTP_DATA87 = crate::Reg<otp_data87::OTP_DATA87rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data87;
/**OTP_DATA88 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data88::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data88::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA88)

For information about available fields see [`mod@otp_data88`] module*/
pub type OTP_DATA88 = crate::Reg<otp_data88::OTP_DATA88rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data88;
/**OTP_DATA89 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data89::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data89::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA89)

For information about available fields see [`mod@otp_data89`] module*/
pub type OTP_DATA89 = crate::Reg<otp_data89::OTP_DATA89rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data89;
/**OTP_DATA90 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data90::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data90::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA90)

For information about available fields see [`mod@otp_data90`] module*/
pub type OTP_DATA90 = crate::Reg<otp_data90::OTP_DATA90rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data90;
/**OTP_DATA91 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data91::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data91::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA91)

For information about available fields see [`mod@otp_data91`] module*/
pub type OTP_DATA91 = crate::Reg<otp_data91::OTP_DATA91rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data91;
/**OTP_DATA92 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data92::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data92::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA92)

For information about available fields see [`mod@otp_data92`] module*/
pub type OTP_DATA92 = crate::Reg<otp_data92::OTP_DATA92rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data92;
/**OTP_DATA93 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data93::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data93::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA93)

For information about available fields see [`mod@otp_data93`] module*/
pub type OTP_DATA93 = crate::Reg<otp_data93::OTP_DATA93rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data93;
/**OTP_DATA94 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data94::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data94::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA94)

For information about available fields see [`mod@otp_data94`] module*/
pub type OTP_DATA94 = crate::Reg<otp_data94::OTP_DATA94rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data94;
/**OTP_DATA95 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data95::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data95::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA95)

For information about available fields see [`mod@otp_data95`] module*/
pub type OTP_DATA95 = crate::Reg<otp_data95::OTP_DATA95rs>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod otp_data95;
/**HWCFGR (r) register accessor: BSEC hardware configuration register

You can [`read`](crate::Reg::read) this register and get [`hwcfgr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:HWCFGR)

For information about available fields see [`mod@hwcfgr`] module*/
pub type HWCFGR = crate::Reg<hwcfgr::HWCFGRrs>;
///BSEC hardware configuration register
pub mod hwcfgr;
/**VERR (r) register accessor: BSEC version register

You can [`read`](crate::Reg::read) this register and get [`verr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:VERR)

For information about available fields see [`mod@verr`] module*/
pub type VERR = crate::Reg<verr::VERRrs>;
///BSEC version register
pub mod verr;
/**IPIDR (r) register accessor: BSEC identification register

You can [`read`](crate::Reg::read) this register and get [`ipidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:IPIDR)

For information about available fields see [`mod@ipidr`] module*/
pub type IPIDR = crate::Reg<ipidr::IPIDRrs>;
///BSEC identification register
pub mod ipidr;
/**SIDR (r) register accessor: BSEC size identification register

You can [`read`](crate::Reg::read) this register and get [`sidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:SIDR)

For information about available fields see [`mod@sidr`] module*/
pub type SIDR = crate::Reg<sidr::SIDRrs>;
///BSEC size identification register
pub mod sidr;
