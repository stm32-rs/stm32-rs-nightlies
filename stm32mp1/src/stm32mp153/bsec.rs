#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    bsec_otp_config: BSEC_OTP_CONFIG,
    bsec_otp_control: BSEC_OTP_CONTROL,
    bsec_otp_wrdata: BSEC_OTP_WRDATA,
    bsec_otp_status: BSEC_OTP_STATUS,
    bsec_otp_lock: BSEC_OTP_LOCK,
    bsec_denable: BSEC_DENABLE,
    _reserved6: [u8; 0x04],
    bsec_otp_disturbed0: BSEC_OTP_DISTURBED0,
    bsec_otp_disturbed1: BSEC_OTP_DISTURBED1,
    bsec_otp_disturbed2: BSEC_OTP_DISTURBED2,
    _reserved9: [u8; 0x0c],
    bsec_otp_error0: BSEC_OTP_ERROR0,
    bsec_otp_error1: BSEC_OTP_ERROR1,
    bsec_otp_error2: BSEC_OTP_ERROR2,
    _reserved12: [u8; 0x0c],
    bsec_otp_wrlock0: BSEC_OTP_WRLOCK0,
    bsec_otp_wrlock1: BSEC_OTP_WRLOCK1,
    bsec_otp_wrlock2: BSEC_OTP_WRLOCK2,
    _reserved15: [u8; 0x0c],
    bsec_otp_splock0: BSEC_OTP_SPLOCK0,
    bsec_otp_splock1: BSEC_OTP_SPLOCK1,
    bsec_otp_splock2: BSEC_OTP_SPLOCK2,
    _reserved18: [u8; 0x0c],
    bsec_otp_swlock0: BSEC_OTP_SWLOCK0,
    bsec_otp_swlock1: BSEC_OTP_SWLOCK1,
    bsec_otp_swlock2: BSEC_OTP_SWLOCK2,
    _reserved21: [u8; 0x0c],
    bsec_otp_srlock0: BSEC_OTP_SRLOCK0,
    bsec_otp_srlock1: BSEC_OTP_SRLOCK1,
    bsec_otp_srlock2: BSEC_OTP_SRLOCK2,
    _reserved24: [u8; 0x0c],
    bsec_jtagin: BSEC_JTAGIN,
    bsec_jtagout: BSEC_JTAGOUT,
    bsec_scratch: BSEC_SCRATCH,
    _reserved27: [u8; 0x0148],
    bsec_otp_data0: BSEC_OTP_DATA0,
    bsec_otp_data1: BSEC_OTP_DATA1,
    bsec_otp_data2: BSEC_OTP_DATA2,
    bsec_otp_data3: BSEC_OTP_DATA3,
    bsec_otp_data4: BSEC_OTP_DATA4,
    bsec_otp_data5: BSEC_OTP_DATA5,
    bsec_otp_data6: BSEC_OTP_DATA6,
    bsec_otp_data7: BSEC_OTP_DATA7,
    bsec_otp_data8: BSEC_OTP_DATA8,
    bsec_otp_data9: BSEC_OTP_DATA9,
    bsec_otp_data10: BSEC_OTP_DATA10,
    bsec_otp_data11: BSEC_OTP_DATA11,
    bsec_otp_data12: BSEC_OTP_DATA12,
    bsec_otp_data13: BSEC_OTP_DATA13,
    bsec_otp_data14: BSEC_OTP_DATA14,
    bsec_otp_data15: BSEC_OTP_DATA15,
    bsec_otp_data16: BSEC_OTP_DATA16,
    bsec_otp_data17: BSEC_OTP_DATA17,
    bsec_otp_data18: BSEC_OTP_DATA18,
    bsec_otp_data19: BSEC_OTP_DATA19,
    bsec_otp_data20: BSEC_OTP_DATA20,
    bsec_otp_data21: BSEC_OTP_DATA21,
    bsec_otp_data22: BSEC_OTP_DATA22,
    bsec_otp_data23: BSEC_OTP_DATA23,
    bsec_otp_data24: BSEC_OTP_DATA24,
    bsec_otp_data25: BSEC_OTP_DATA25,
    bsec_otp_data26: BSEC_OTP_DATA26,
    bsec_otp_data27: BSEC_OTP_DATA27,
    bsec_otp_data28: BSEC_OTP_DATA28,
    bsec_otp_data29: BSEC_OTP_DATA29,
    bsec_otp_data30: BSEC_OTP_DATA30,
    bsec_otp_data31: BSEC_OTP_DATA31,
    bsec_otp_data32: BSEC_OTP_DATA32,
    bsec_otp_data33: BSEC_OTP_DATA33,
    bsec_otp_data34: BSEC_OTP_DATA34,
    bsec_otp_data35: BSEC_OTP_DATA35,
    bsec_otp_data36: BSEC_OTP_DATA36,
    bsec_otp_data37: BSEC_OTP_DATA37,
    bsec_otp_data38: BSEC_OTP_DATA38,
    bsec_otp_data39: BSEC_OTP_DATA39,
    bsec_otp_data40: BSEC_OTP_DATA40,
    bsec_otp_data41: BSEC_OTP_DATA41,
    bsec_otp_data42: BSEC_OTP_DATA42,
    bsec_otp_data43: BSEC_OTP_DATA43,
    bsec_otp_data44: BSEC_OTP_DATA44,
    bsec_otp_data45: BSEC_OTP_DATA45,
    bsec_otp_data46: BSEC_OTP_DATA46,
    bsec_otp_data47: BSEC_OTP_DATA47,
    bsec_otp_data48: BSEC_OTP_DATA48,
    bsec_otp_data49: BSEC_OTP_DATA49,
    bsec_otp_data50: BSEC_OTP_DATA50,
    bsec_otp_data51: BSEC_OTP_DATA51,
    bsec_otp_data52: BSEC_OTP_DATA52,
    bsec_otp_data53: BSEC_OTP_DATA53,
    bsec_otp_data54: BSEC_OTP_DATA54,
    bsec_otp_data55: BSEC_OTP_DATA55,
    bsec_otp_data56: BSEC_OTP_DATA56,
    bsec_otp_data57: BSEC_OTP_DATA57,
    bsec_otp_data58: BSEC_OTP_DATA58,
    bsec_otp_data59: BSEC_OTP_DATA59,
    bsec_otp_data60: BSEC_OTP_DATA60,
    bsec_otp_data61: BSEC_OTP_DATA61,
    bsec_otp_data62: BSEC_OTP_DATA62,
    bsec_otp_data63: BSEC_OTP_DATA63,
    bsec_otp_data64: BSEC_OTP_DATA64,
    bsec_otp_data65: BSEC_OTP_DATA65,
    bsec_otp_data66: BSEC_OTP_DATA66,
    bsec_otp_data67: BSEC_OTP_DATA67,
    bsec_otp_data68: BSEC_OTP_DATA68,
    bsec_otp_data69: BSEC_OTP_DATA69,
    bsec_otp_data70: BSEC_OTP_DATA70,
    bsec_otp_data71: BSEC_OTP_DATA71,
    bsec_otp_data72: BSEC_OTP_DATA72,
    bsec_otp_data73: BSEC_OTP_DATA73,
    bsec_otp_data74: BSEC_OTP_DATA74,
    bsec_otp_data75: BSEC_OTP_DATA75,
    bsec_otp_data76: BSEC_OTP_DATA76,
    bsec_otp_data77: BSEC_OTP_DATA77,
    bsec_otp_data78: BSEC_OTP_DATA78,
    bsec_otp_data79: BSEC_OTP_DATA79,
    bsec_otp_data80: BSEC_OTP_DATA80,
    bsec_otp_data81: BSEC_OTP_DATA81,
    bsec_otp_data82: BSEC_OTP_DATA82,
    bsec_otp_data83: BSEC_OTP_DATA83,
    bsec_otp_data84: BSEC_OTP_DATA84,
    bsec_otp_data85: BSEC_OTP_DATA85,
    bsec_otp_data86: BSEC_OTP_DATA86,
    bsec_otp_data87: BSEC_OTP_DATA87,
    bsec_otp_data88: BSEC_OTP_DATA88,
    bsec_otp_data89: BSEC_OTP_DATA89,
    bsec_otp_data90: BSEC_OTP_DATA90,
    bsec_otp_data91: BSEC_OTP_DATA91,
    bsec_otp_data92: BSEC_OTP_DATA92,
    bsec_otp_data93: BSEC_OTP_DATA93,
    bsec_otp_data94: BSEC_OTP_DATA94,
    bsec_otp_data95: BSEC_OTP_DATA95,
    _reserved123: [u8; 0x0c70],
    bsec_hwcfgr: BSEC_HWCFGR,
    bsec_verr: BSEC_VERR,
    bsec_ipidr: BSEC_IPIDR,
    bsec_sidr: BSEC_SIDR,
}
impl RegisterBlock {
    ///0x00 - BSEC OTP configuration register
    #[inline(always)]
    pub const fn bsec_otp_config(&self) -> &BSEC_OTP_CONFIG {
        &self.bsec_otp_config
    }
    ///0x04 - BSEC OTP control register
    #[inline(always)]
    pub const fn bsec_otp_control(&self) -> &BSEC_OTP_CONTROL {
        &self.bsec_otp_control
    }
    ///0x08 - BSEC OTP write data register
    #[inline(always)]
    pub const fn bsec_otp_wrdata(&self) -> &BSEC_OTP_WRDATA {
        &self.bsec_otp_wrdata
    }
    ///0x0c - BSEC OTP status register
    #[inline(always)]
    pub const fn bsec_otp_status(&self) -> &BSEC_OTP_STATUS {
        &self.bsec_otp_status
    }
    ///0x10 - BSEC OTP lock configuration register
    #[inline(always)]
    pub const fn bsec_otp_lock(&self) -> &BSEC_OTP_LOCK {
        &self.bsec_otp_lock
    }
    ///0x14 - reset value depends on OTP secure mode according toTable18: BSEC_DENABLE default values after reset on page181.
    #[inline(always)]
    pub const fn bsec_denable(&self) -> &BSEC_DENABLE {
        &self.bsec_denable
    }
    ///0x1c - BSEC_OTP_DISTURBED0 is used to report disturbed state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP). BSEC_OTP_DISTURBED1 is used to report disturbed state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_DISTURBED2 is used to report disturbed state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95.
    #[inline(always)]
    pub const fn bsec_otp_disturbed0(&self) -> &BSEC_OTP_DISTURBED0 {
        &self.bsec_otp_disturbed0
    }
    ///0x20 - BSEC_OTP_DISTURBED0 is used to report disturbed state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP). BSEC_OTP_DISTURBED1 is used to report disturbed state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_DISTURBED2 is used to report disturbed state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95.
    #[inline(always)]
    pub const fn bsec_otp_disturbed1(&self) -> &BSEC_OTP_DISTURBED1 {
        &self.bsec_otp_disturbed1
    }
    ///0x24 - BSEC_OTP_DISTURBED0 is used to report disturbed state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP). BSEC_OTP_DISTURBED1 is used to report disturbed state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_DISTURBED2 is used to report disturbed state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95.
    #[inline(always)]
    pub const fn bsec_otp_disturbed2(&self) -> &BSEC_OTP_DISTURBED2 {
        &self.bsec_otp_disturbed2
    }
    ///0x34 - BSEC_OTP_ERROR0 is used to report error state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP which are protected by 2:1 redundancy). BSEC_OTP_ERROR1 is used to report error state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 which are protected by 6-bit ECC. BSEC_OTP_ERROR2 is used to report error state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 which are protected by 6-bit ECC.
    #[inline(always)]
    pub const fn bsec_otp_error0(&self) -> &BSEC_OTP_ERROR0 {
        &self.bsec_otp_error0
    }
    ///0x38 - BSEC_OTP_ERROR0 is used to report error state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP which are protected by 2:1 redundancy). BSEC_OTP_ERROR1 is used to report error state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 which are protected by 6-bit ECC. BSEC_OTP_ERROR2 is used to report error state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 which are protected by 6-bit ECC.
    #[inline(always)]
    pub const fn bsec_otp_error1(&self) -> &BSEC_OTP_ERROR1 {
        &self.bsec_otp_error1
    }
    ///0x3c - BSEC_OTP_ERROR0 is used to report error state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP which are protected by 2:1 redundancy). BSEC_OTP_ERROR1 is used to report error state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 which are protected by 6-bit ECC. BSEC_OTP_ERROR2 is used to report error state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 which are protected by 6-bit ECC.
    #[inline(always)]
    pub const fn bsec_otp_error2(&self) -> &BSEC_OTP_ERROR2 {
        &self.bsec_otp_error2
    }
    ///0x4c - BSEC_OTP_WLOCK0 is used to report permanent write lock of BSEC_OTP_DATA0 to BSEC_OTP_DATA31. BSEC_OTP_WLOCK1 is used to report permanent write lock of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_WLOCK2 is used to report permanent write lock of BSEC_OTP_DATA64 to BSEC_OTP_DATA95. Permanent write lock requires a programming sequence to lock a word (see section:Section3.3.6: OTP operations on page178).
    #[inline(always)]
    pub const fn bsec_otp_wrlock0(&self) -> &BSEC_OTP_WRLOCK0 {
        &self.bsec_otp_wrlock0
    }
    ///0x50 - BSEC_OTP_WLOCK0 is used to report permanent write lock of BSEC_OTP_DATA0 to BSEC_OTP_DATA31. BSEC_OTP_WLOCK1 is used to report permanent write lock of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_WLOCK2 is used to report permanent write lock of BSEC_OTP_DATA64 to BSEC_OTP_DATA95. Permanent write lock requires a programming sequence to lock a word (see section:Section3.3.6: OTP operations on page178).
    #[inline(always)]
    pub const fn bsec_otp_wrlock1(&self) -> &BSEC_OTP_WRLOCK1 {
        &self.bsec_otp_wrlock1
    }
    ///0x54 - BSEC_OTP_WLOCK0 is used to report permanent write lock of BSEC_OTP_DATA0 to BSEC_OTP_DATA31. BSEC_OTP_WLOCK1 is used to report permanent write lock of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_WLOCK2 is used to report permanent write lock of BSEC_OTP_DATA64 to BSEC_OTP_DATA95. Permanent write lock requires a programming sequence to lock a word (see section:Section3.3.6: OTP operations on page178).
    #[inline(always)]
    pub const fn bsec_otp_wrlock2(&self) -> &BSEC_OTP_WRLOCK2 {
        &self.bsec_otp_wrlock2
    }
    ///0x64 - BSEC_OTP_SPLOCK0 is used to lock the programming of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset BSEC_OTP_SPLOCK1 is used to lock the programming of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset BSEC_OTP_SPLOCK2 is used to lock the programming of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset Attempt to sticky program locked OTP word are silently ignored.
    #[inline(always)]
    pub const fn bsec_otp_splock0(&self) -> &BSEC_OTP_SPLOCK0 {
        &self.bsec_otp_splock0
    }
    ///0x68 - BSEC_OTP_SPLOCK0 is used to lock the programming of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset BSEC_OTP_SPLOCK1 is used to lock the programming of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset BSEC_OTP_SPLOCK2 is used to lock the programming of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset Attempt to sticky program locked OTP word are silently ignored.
    #[inline(always)]
    pub const fn bsec_otp_splock1(&self) -> &BSEC_OTP_SPLOCK1 {
        &self.bsec_otp_splock1
    }
    ///0x6c - BSEC_OTP_SPLOCK0 is used to lock the programming of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset BSEC_OTP_SPLOCK1 is used to lock the programming of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset BSEC_OTP_SPLOCK2 is used to lock the programming of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset Attempt to sticky program locked OTP word are silently ignored.
    #[inline(always)]
    pub const fn bsec_otp_splock2(&self) -> &BSEC_OTP_SPLOCK2 {
        &self.bsec_otp_splock2
    }
    ///0x7c - BSEC_OTP_SWLOCK0 is used to prevent writing to BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SWLOCK1 is used to prevent writing to BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SWLOCK2 is used to prevent writing to BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Write to shadow write locked BSEC_OTP_DATA word are silently ignored. Writing to OTP word 0 shadow is always prevented.
    #[inline(always)]
    pub const fn bsec_otp_swlock0(&self) -> &BSEC_OTP_SWLOCK0 {
        &self.bsec_otp_swlock0
    }
    ///0x80 - BSEC_OTP_SWLOCK0 is used to prevent writing to BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SWLOCK1 is used to prevent writing to BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SWLOCK2 is used to prevent writing to BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Write to shadow write locked BSEC_OTP_DATA word are silently ignored. Writing to OTP word 0 shadow is always prevented.
    #[inline(always)]
    pub const fn bsec_otp_swlock1(&self) -> &BSEC_OTP_SWLOCK1 {
        &self.bsec_otp_swlock1
    }
    ///0x84 - BSEC_OTP_SWLOCK0 is used to prevent writing to BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SWLOCK1 is used to prevent writing to BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SWLOCK2 is used to prevent writing to BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Write to shadow write locked BSEC_OTP_DATA word are silently ignored. Writing to OTP word 0 shadow is always prevented.
    #[inline(always)]
    pub const fn bsec_otp_swlock2(&self) -> &BSEC_OTP_SWLOCK2 {
        &self.bsec_otp_swlock2
    }
    ///0x94 - BSEC_OTP_SRLOCK0 is used to prevent reloading of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SRLOCK1 is used to prevent reloading of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SRLOCK2 is used to prevent reloading of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Setting SRLOCK bits or attempt to reload a locked OTP do not clear the corresponding BSEC_OTP_DATAx shadow register. BSEC_OTP_SRLOCK0 bit 0 is controlled by hardware according to fuse_ok, writing to this bit has no effect.
    #[inline(always)]
    pub const fn bsec_otp_srlock0(&self) -> &BSEC_OTP_SRLOCK0 {
        &self.bsec_otp_srlock0
    }
    ///0x98 - BSEC_OTP_SRLOCK0 is used to prevent reloading of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SRLOCK1 is used to prevent reloading of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SRLOCK2 is used to prevent reloading of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Setting SRLOCK bits or attempt to reload a locked OTP do not clear the corresponding BSEC_OTP_DATAx shadow register. BSEC_OTP_SRLOCK0 bit 0 is controlled by hardware according to fuse_ok, writing to this bit has no effect.
    #[inline(always)]
    pub const fn bsec_otp_srlock1(&self) -> &BSEC_OTP_SRLOCK1 {
        &self.bsec_otp_srlock1
    }
    ///0x9c - BSEC_OTP_SRLOCK0 is used to prevent reloading of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SRLOCK1 is used to prevent reloading of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SRLOCK2 is used to prevent reloading of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Setting SRLOCK bits or attempt to reload a locked OTP do not clear the corresponding BSEC_OTP_DATAx shadow register. BSEC_OTP_SRLOCK0 bit 0 is controlled by hardware according to fuse_ok, writing to this bit has no effect.
    #[inline(always)]
    pub const fn bsec_otp_srlock2(&self) -> &BSEC_OTP_SRLOCK2 {
        &self.bsec_otp_srlock2
    }
    ///0xac - BSEC JTAG input register
    #[inline(always)]
    pub const fn bsec_jtagin(&self) -> &BSEC_JTAGIN {
        &self.bsec_jtagin
    }
    ///0xb0 - BSEC JTAG output register
    #[inline(always)]
    pub const fn bsec_jtagout(&self) -> &BSEC_JTAGOUT {
        &self.bsec_jtagout
    }
    ///0xb4 - BSEC scratch register
    #[inline(always)]
    pub const fn bsec_scratch(&self) -> &BSEC_SCRATCH {
        &self.bsec_scratch
    }
    /**0x200 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data0(&self) -> &BSEC_OTP_DATA0 {
        &self.bsec_otp_data0
    }
    /**0x204 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data1(&self) -> &BSEC_OTP_DATA1 {
        &self.bsec_otp_data1
    }
    /**0x208 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data2(&self) -> &BSEC_OTP_DATA2 {
        &self.bsec_otp_data2
    }
    /**0x20c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data3(&self) -> &BSEC_OTP_DATA3 {
        &self.bsec_otp_data3
    }
    /**0x210 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data4(&self) -> &BSEC_OTP_DATA4 {
        &self.bsec_otp_data4
    }
    /**0x214 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data5(&self) -> &BSEC_OTP_DATA5 {
        &self.bsec_otp_data5
    }
    /**0x218 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data6(&self) -> &BSEC_OTP_DATA6 {
        &self.bsec_otp_data6
    }
    /**0x21c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data7(&self) -> &BSEC_OTP_DATA7 {
        &self.bsec_otp_data7
    }
    /**0x220 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data8(&self) -> &BSEC_OTP_DATA8 {
        &self.bsec_otp_data8
    }
    /**0x224 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data9(&self) -> &BSEC_OTP_DATA9 {
        &self.bsec_otp_data9
    }
    /**0x228 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data10(&self) -> &BSEC_OTP_DATA10 {
        &self.bsec_otp_data10
    }
    /**0x22c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data11(&self) -> &BSEC_OTP_DATA11 {
        &self.bsec_otp_data11
    }
    /**0x230 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data12(&self) -> &BSEC_OTP_DATA12 {
        &self.bsec_otp_data12
    }
    /**0x234 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data13(&self) -> &BSEC_OTP_DATA13 {
        &self.bsec_otp_data13
    }
    /**0x238 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data14(&self) -> &BSEC_OTP_DATA14 {
        &self.bsec_otp_data14
    }
    /**0x23c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data15(&self) -> &BSEC_OTP_DATA15 {
        &self.bsec_otp_data15
    }
    /**0x240 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data16(&self) -> &BSEC_OTP_DATA16 {
        &self.bsec_otp_data16
    }
    /**0x244 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data17(&self) -> &BSEC_OTP_DATA17 {
        &self.bsec_otp_data17
    }
    /**0x248 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data18(&self) -> &BSEC_OTP_DATA18 {
        &self.bsec_otp_data18
    }
    /**0x24c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data19(&self) -> &BSEC_OTP_DATA19 {
        &self.bsec_otp_data19
    }
    /**0x250 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data20(&self) -> &BSEC_OTP_DATA20 {
        &self.bsec_otp_data20
    }
    /**0x254 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data21(&self) -> &BSEC_OTP_DATA21 {
        &self.bsec_otp_data21
    }
    /**0x258 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data22(&self) -> &BSEC_OTP_DATA22 {
        &self.bsec_otp_data22
    }
    /**0x25c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data23(&self) -> &BSEC_OTP_DATA23 {
        &self.bsec_otp_data23
    }
    /**0x260 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data24(&self) -> &BSEC_OTP_DATA24 {
        &self.bsec_otp_data24
    }
    /**0x264 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data25(&self) -> &BSEC_OTP_DATA25 {
        &self.bsec_otp_data25
    }
    /**0x268 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data26(&self) -> &BSEC_OTP_DATA26 {
        &self.bsec_otp_data26
    }
    /**0x26c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data27(&self) -> &BSEC_OTP_DATA27 {
        &self.bsec_otp_data27
    }
    /**0x270 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data28(&self) -> &BSEC_OTP_DATA28 {
        &self.bsec_otp_data28
    }
    /**0x274 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data29(&self) -> &BSEC_OTP_DATA29 {
        &self.bsec_otp_data29
    }
    /**0x278 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data30(&self) -> &BSEC_OTP_DATA30 {
        &self.bsec_otp_data30
    }
    /**0x27c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data31(&self) -> &BSEC_OTP_DATA31 {
        &self.bsec_otp_data31
    }
    /**0x280 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data32(&self) -> &BSEC_OTP_DATA32 {
        &self.bsec_otp_data32
    }
    /**0x284 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data33(&self) -> &BSEC_OTP_DATA33 {
        &self.bsec_otp_data33
    }
    /**0x288 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data34(&self) -> &BSEC_OTP_DATA34 {
        &self.bsec_otp_data34
    }
    /**0x28c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data35(&self) -> &BSEC_OTP_DATA35 {
        &self.bsec_otp_data35
    }
    /**0x290 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data36(&self) -> &BSEC_OTP_DATA36 {
        &self.bsec_otp_data36
    }
    /**0x294 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data37(&self) -> &BSEC_OTP_DATA37 {
        &self.bsec_otp_data37
    }
    /**0x298 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data38(&self) -> &BSEC_OTP_DATA38 {
        &self.bsec_otp_data38
    }
    /**0x29c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data39(&self) -> &BSEC_OTP_DATA39 {
        &self.bsec_otp_data39
    }
    /**0x2a0 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data40(&self) -> &BSEC_OTP_DATA40 {
        &self.bsec_otp_data40
    }
    /**0x2a4 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data41(&self) -> &BSEC_OTP_DATA41 {
        &self.bsec_otp_data41
    }
    /**0x2a8 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data42(&self) -> &BSEC_OTP_DATA42 {
        &self.bsec_otp_data42
    }
    /**0x2ac - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data43(&self) -> &BSEC_OTP_DATA43 {
        &self.bsec_otp_data43
    }
    /**0x2b0 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data44(&self) -> &BSEC_OTP_DATA44 {
        &self.bsec_otp_data44
    }
    /**0x2b4 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data45(&self) -> &BSEC_OTP_DATA45 {
        &self.bsec_otp_data45
    }
    /**0x2b8 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data46(&self) -> &BSEC_OTP_DATA46 {
        &self.bsec_otp_data46
    }
    /**0x2bc - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data47(&self) -> &BSEC_OTP_DATA47 {
        &self.bsec_otp_data47
    }
    /**0x2c0 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data48(&self) -> &BSEC_OTP_DATA48 {
        &self.bsec_otp_data48
    }
    /**0x2c4 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data49(&self) -> &BSEC_OTP_DATA49 {
        &self.bsec_otp_data49
    }
    /**0x2c8 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data50(&self) -> &BSEC_OTP_DATA50 {
        &self.bsec_otp_data50
    }
    /**0x2cc - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data51(&self) -> &BSEC_OTP_DATA51 {
        &self.bsec_otp_data51
    }
    /**0x2d0 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data52(&self) -> &BSEC_OTP_DATA52 {
        &self.bsec_otp_data52
    }
    /**0x2d4 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data53(&self) -> &BSEC_OTP_DATA53 {
        &self.bsec_otp_data53
    }
    /**0x2d8 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data54(&self) -> &BSEC_OTP_DATA54 {
        &self.bsec_otp_data54
    }
    /**0x2dc - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data55(&self) -> &BSEC_OTP_DATA55 {
        &self.bsec_otp_data55
    }
    /**0x2e0 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data56(&self) -> &BSEC_OTP_DATA56 {
        &self.bsec_otp_data56
    }
    /**0x2e4 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data57(&self) -> &BSEC_OTP_DATA57 {
        &self.bsec_otp_data57
    }
    /**0x2e8 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data58(&self) -> &BSEC_OTP_DATA58 {
        &self.bsec_otp_data58
    }
    /**0x2ec - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data59(&self) -> &BSEC_OTP_DATA59 {
        &self.bsec_otp_data59
    }
    /**0x2f0 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data60(&self) -> &BSEC_OTP_DATA60 {
        &self.bsec_otp_data60
    }
    /**0x2f4 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data61(&self) -> &BSEC_OTP_DATA61 {
        &self.bsec_otp_data61
    }
    /**0x2f8 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data62(&self) -> &BSEC_OTP_DATA62 {
        &self.bsec_otp_data62
    }
    /**0x2fc - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data63(&self) -> &BSEC_OTP_DATA63 {
        &self.bsec_otp_data63
    }
    /**0x300 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data64(&self) -> &BSEC_OTP_DATA64 {
        &self.bsec_otp_data64
    }
    /**0x304 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data65(&self) -> &BSEC_OTP_DATA65 {
        &self.bsec_otp_data65
    }
    /**0x308 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data66(&self) -> &BSEC_OTP_DATA66 {
        &self.bsec_otp_data66
    }
    /**0x30c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data67(&self) -> &BSEC_OTP_DATA67 {
        &self.bsec_otp_data67
    }
    /**0x310 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data68(&self) -> &BSEC_OTP_DATA68 {
        &self.bsec_otp_data68
    }
    /**0x314 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data69(&self) -> &BSEC_OTP_DATA69 {
        &self.bsec_otp_data69
    }
    /**0x318 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data70(&self) -> &BSEC_OTP_DATA70 {
        &self.bsec_otp_data70
    }
    /**0x31c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data71(&self) -> &BSEC_OTP_DATA71 {
        &self.bsec_otp_data71
    }
    /**0x320 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data72(&self) -> &BSEC_OTP_DATA72 {
        &self.bsec_otp_data72
    }
    /**0x324 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data73(&self) -> &BSEC_OTP_DATA73 {
        &self.bsec_otp_data73
    }
    /**0x328 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data74(&self) -> &BSEC_OTP_DATA74 {
        &self.bsec_otp_data74
    }
    /**0x32c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data75(&self) -> &BSEC_OTP_DATA75 {
        &self.bsec_otp_data75
    }
    /**0x330 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data76(&self) -> &BSEC_OTP_DATA76 {
        &self.bsec_otp_data76
    }
    /**0x334 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data77(&self) -> &BSEC_OTP_DATA77 {
        &self.bsec_otp_data77
    }
    /**0x338 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data78(&self) -> &BSEC_OTP_DATA78 {
        &self.bsec_otp_data78
    }
    /**0x33c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data79(&self) -> &BSEC_OTP_DATA79 {
        &self.bsec_otp_data79
    }
    /**0x340 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data80(&self) -> &BSEC_OTP_DATA80 {
        &self.bsec_otp_data80
    }
    /**0x344 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data81(&self) -> &BSEC_OTP_DATA81 {
        &self.bsec_otp_data81
    }
    /**0x348 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data82(&self) -> &BSEC_OTP_DATA82 {
        &self.bsec_otp_data82
    }
    /**0x34c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data83(&self) -> &BSEC_OTP_DATA83 {
        &self.bsec_otp_data83
    }
    /**0x350 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data84(&self) -> &BSEC_OTP_DATA84 {
        &self.bsec_otp_data84
    }
    /**0x354 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data85(&self) -> &BSEC_OTP_DATA85 {
        &self.bsec_otp_data85
    }
    /**0x358 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data86(&self) -> &BSEC_OTP_DATA86 {
        &self.bsec_otp_data86
    }
    /**0x35c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data87(&self) -> &BSEC_OTP_DATA87 {
        &self.bsec_otp_data87
    }
    /**0x360 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data88(&self) -> &BSEC_OTP_DATA88 {
        &self.bsec_otp_data88
    }
    /**0x364 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data89(&self) -> &BSEC_OTP_DATA89 {
        &self.bsec_otp_data89
    }
    /**0x368 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data90(&self) -> &BSEC_OTP_DATA90 {
        &self.bsec_otp_data90
    }
    /**0x36c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data91(&self) -> &BSEC_OTP_DATA91 {
        &self.bsec_otp_data91
    }
    /**0x370 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data92(&self) -> &BSEC_OTP_DATA92 {
        &self.bsec_otp_data92
    }
    /**0x374 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data93(&self) -> &BSEC_OTP_DATA93 {
        &self.bsec_otp_data93
    }
    /**0x378 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data94(&self) -> &BSEC_OTP_DATA94 {
        &self.bsec_otp_data94
    }
    /**0x37c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
    #[inline(always)]
    pub const fn bsec_otp_data95(&self) -> &BSEC_OTP_DATA95 {
        &self.bsec_otp_data95
    }
    ///0xff0 - BSEC hardware configuration register
    #[inline(always)]
    pub const fn bsec_hwcfgr(&self) -> &BSEC_HWCFGR {
        &self.bsec_hwcfgr
    }
    ///0xff4 - BSEC version register
    #[inline(always)]
    pub const fn bsec_verr(&self) -> &BSEC_VERR {
        &self.bsec_verr
    }
    ///0xff8 - BSEC identification register
    #[inline(always)]
    pub const fn bsec_ipidr(&self) -> &BSEC_IPIDR {
        &self.bsec_ipidr
    }
    ///0xffc - BSEC size identification register
    #[inline(always)]
    pub const fn bsec_sidr(&self) -> &BSEC_SIDR {
        &self.bsec_sidr
    }
}
/**BSEC_OTP_CONFIG (rw) register accessor: BSEC OTP configuration register

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_CONFIG)

For information about available fields see [`mod@bsec_otp_config`]
module*/
pub type BSEC_OTP_CONFIG = crate::Reg<bsec_otp_config::BSEC_OTP_CONFIGrs>;
///BSEC OTP configuration register
pub mod bsec_otp_config;
/**BSEC_OTP_CONTROL (rw) register accessor: BSEC OTP control register

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_CONTROL)

For information about available fields see [`mod@bsec_otp_control`]
module*/
pub type BSEC_OTP_CONTROL = crate::Reg<bsec_otp_control::BSEC_OTP_CONTROLrs>;
///BSEC OTP control register
pub mod bsec_otp_control;
/**BSEC_OTP_WRDATA (rw) register accessor: BSEC OTP write data register

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_wrdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_wrdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_WRDATA)

For information about available fields see [`mod@bsec_otp_wrdata`]
module*/
pub type BSEC_OTP_WRDATA = crate::Reg<bsec_otp_wrdata::BSEC_OTP_WRDATArs>;
///BSEC OTP write data register
pub mod bsec_otp_wrdata;
/**BSEC_OTP_STATUS (r) register accessor: BSEC OTP status register

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_STATUS)

For information about available fields see [`mod@bsec_otp_status`]
module*/
pub type BSEC_OTP_STATUS = crate::Reg<bsec_otp_status::BSEC_OTP_STATUSrs>;
///BSEC OTP status register
pub mod bsec_otp_status;
/**BSEC_OTP_LOCK (rw) register accessor: BSEC OTP lock configuration register

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_lock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_lock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_LOCK)

For information about available fields see [`mod@bsec_otp_lock`]
module*/
pub type BSEC_OTP_LOCK = crate::Reg<bsec_otp_lock::BSEC_OTP_LOCKrs>;
///BSEC OTP lock configuration register
pub mod bsec_otp_lock;
/**BSEC_DENABLE (rw) register accessor: reset value depends on OTP secure mode according toTable18: BSEC_DENABLE default values after reset on page181.

You can [`read`](crate::Reg::read) this register and get [`bsec_denable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_denable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_DENABLE)

For information about available fields see [`mod@bsec_denable`]
module*/
pub type BSEC_DENABLE = crate::Reg<bsec_denable::BSEC_DENABLErs>;
///reset value depends on OTP secure mode according toTable18: BSEC_DENABLE default values after reset on page181.
pub mod bsec_denable;
/**BSEC_OTP_DISTURBED0 (r) register accessor: BSEC_OTP_DISTURBED0 is used to report disturbed state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP). BSEC_OTP_DISTURBED1 is used to report disturbed state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_DISTURBED2 is used to report disturbed state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_disturbed0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DISTURBED0)

For information about available fields see [`mod@bsec_otp_disturbed0`]
module*/
pub type BSEC_OTP_DISTURBED0 = crate::Reg<bsec_otp_disturbed0::BSEC_OTP_DISTURBED0rs>;
///BSEC_OTP_DISTURBED0 is used to report disturbed state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP). BSEC_OTP_DISTURBED1 is used to report disturbed state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_DISTURBED2 is used to report disturbed state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95.
pub mod bsec_otp_disturbed0;
/**BSEC_OTP_DISTURBED1 (r) register accessor: BSEC_OTP_DISTURBED0 is used to report disturbed state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP). BSEC_OTP_DISTURBED1 is used to report disturbed state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_DISTURBED2 is used to report disturbed state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_disturbed1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DISTURBED1)

For information about available fields see [`mod@bsec_otp_disturbed1`]
module*/
pub type BSEC_OTP_DISTURBED1 = crate::Reg<bsec_otp_disturbed1::BSEC_OTP_DISTURBED1rs>;
///BSEC_OTP_DISTURBED0 is used to report disturbed state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP). BSEC_OTP_DISTURBED1 is used to report disturbed state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_DISTURBED2 is used to report disturbed state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95.
pub mod bsec_otp_disturbed1;
/**BSEC_OTP_DISTURBED2 (r) register accessor: BSEC_OTP_DISTURBED0 is used to report disturbed state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP). BSEC_OTP_DISTURBED1 is used to report disturbed state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_DISTURBED2 is used to report disturbed state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_disturbed2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DISTURBED2)

For information about available fields see [`mod@bsec_otp_disturbed2`]
module*/
pub type BSEC_OTP_DISTURBED2 = crate::Reg<bsec_otp_disturbed2::BSEC_OTP_DISTURBED2rs>;
///BSEC_OTP_DISTURBED0 is used to report disturbed state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP). BSEC_OTP_DISTURBED1 is used to report disturbed state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_DISTURBED2 is used to report disturbed state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95.
pub mod bsec_otp_disturbed2;
/**BSEC_OTP_ERROR0 (r) register accessor: BSEC_OTP_ERROR0 is used to report error state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP which are protected by 2:1 redundancy). BSEC_OTP_ERROR1 is used to report error state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 which are protected by 6-bit ECC. BSEC_OTP_ERROR2 is used to report error state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 which are protected by 6-bit ECC.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_error0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_ERROR0)

For information about available fields see [`mod@bsec_otp_error0`]
module*/
pub type BSEC_OTP_ERROR0 = crate::Reg<bsec_otp_error0::BSEC_OTP_ERROR0rs>;
///BSEC_OTP_ERROR0 is used to report error state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP which are protected by 2:1 redundancy). BSEC_OTP_ERROR1 is used to report error state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 which are protected by 6-bit ECC. BSEC_OTP_ERROR2 is used to report error state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 which are protected by 6-bit ECC.
pub mod bsec_otp_error0;
/**BSEC_OTP_ERROR1 (r) register accessor: BSEC_OTP_ERROR0 is used to report error state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP which are protected by 2:1 redundancy). BSEC_OTP_ERROR1 is used to report error state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 which are protected by 6-bit ECC. BSEC_OTP_ERROR2 is used to report error state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 which are protected by 6-bit ECC.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_error1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_ERROR1)

For information about available fields see [`mod@bsec_otp_error1`]
module*/
pub type BSEC_OTP_ERROR1 = crate::Reg<bsec_otp_error1::BSEC_OTP_ERROR1rs>;
///BSEC_OTP_ERROR0 is used to report error state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP which are protected by 2:1 redundancy). BSEC_OTP_ERROR1 is used to report error state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 which are protected by 6-bit ECC. BSEC_OTP_ERROR2 is used to report error state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 which are protected by 6-bit ECC.
pub mod bsec_otp_error1;
/**BSEC_OTP_ERROR2 (r) register accessor: BSEC_OTP_ERROR0 is used to report error state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP which are protected by 2:1 redundancy). BSEC_OTP_ERROR1 is used to report error state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 which are protected by 6-bit ECC. BSEC_OTP_ERROR2 is used to report error state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 which are protected by 6-bit ECC.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_error2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_ERROR2)

For information about available fields see [`mod@bsec_otp_error2`]
module*/
pub type BSEC_OTP_ERROR2 = crate::Reg<bsec_otp_error2::BSEC_OTP_ERROR2rs>;
///BSEC_OTP_ERROR0 is used to report error state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP which are protected by 2:1 redundancy). BSEC_OTP_ERROR1 is used to report error state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 which are protected by 6-bit ECC. BSEC_OTP_ERROR2 is used to report error state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 which are protected by 6-bit ECC.
pub mod bsec_otp_error2;
/**BSEC_OTP_WRLOCK0 (r) register accessor: BSEC_OTP_WLOCK0 is used to report permanent write lock of BSEC_OTP_DATA0 to BSEC_OTP_DATA31. BSEC_OTP_WLOCK1 is used to report permanent write lock of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_WLOCK2 is used to report permanent write lock of BSEC_OTP_DATA64 to BSEC_OTP_DATA95. Permanent write lock requires a programming sequence to lock a word (see section:Section3.3.6: OTP operations on page178).

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_wrlock0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_WRLOCK0)

For information about available fields see [`mod@bsec_otp_wrlock0`]
module*/
pub type BSEC_OTP_WRLOCK0 = crate::Reg<bsec_otp_wrlock0::BSEC_OTP_WRLOCK0rs>;
///BSEC_OTP_WLOCK0 is used to report permanent write lock of BSEC_OTP_DATA0 to BSEC_OTP_DATA31. BSEC_OTP_WLOCK1 is used to report permanent write lock of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_WLOCK2 is used to report permanent write lock of BSEC_OTP_DATA64 to BSEC_OTP_DATA95. Permanent write lock requires a programming sequence to lock a word (see section:Section3.3.6: OTP operations on page178).
pub mod bsec_otp_wrlock0;
/**BSEC_OTP_WRLOCK1 (r) register accessor: BSEC_OTP_WLOCK0 is used to report permanent write lock of BSEC_OTP_DATA0 to BSEC_OTP_DATA31. BSEC_OTP_WLOCK1 is used to report permanent write lock of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_WLOCK2 is used to report permanent write lock of BSEC_OTP_DATA64 to BSEC_OTP_DATA95. Permanent write lock requires a programming sequence to lock a word (see section:Section3.3.6: OTP operations on page178).

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_wrlock1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_WRLOCK1)

For information about available fields see [`mod@bsec_otp_wrlock1`]
module*/
pub type BSEC_OTP_WRLOCK1 = crate::Reg<bsec_otp_wrlock1::BSEC_OTP_WRLOCK1rs>;
///BSEC_OTP_WLOCK0 is used to report permanent write lock of BSEC_OTP_DATA0 to BSEC_OTP_DATA31. BSEC_OTP_WLOCK1 is used to report permanent write lock of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_WLOCK2 is used to report permanent write lock of BSEC_OTP_DATA64 to BSEC_OTP_DATA95. Permanent write lock requires a programming sequence to lock a word (see section:Section3.3.6: OTP operations on page178).
pub mod bsec_otp_wrlock1;
/**BSEC_OTP_WRLOCK2 (r) register accessor: BSEC_OTP_WLOCK0 is used to report permanent write lock of BSEC_OTP_DATA0 to BSEC_OTP_DATA31. BSEC_OTP_WLOCK1 is used to report permanent write lock of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_WLOCK2 is used to report permanent write lock of BSEC_OTP_DATA64 to BSEC_OTP_DATA95. Permanent write lock requires a programming sequence to lock a word (see section:Section3.3.6: OTP operations on page178).

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_wrlock2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_WRLOCK2)

For information about available fields see [`mod@bsec_otp_wrlock2`]
module*/
pub type BSEC_OTP_WRLOCK2 = crate::Reg<bsec_otp_wrlock2::BSEC_OTP_WRLOCK2rs>;
///BSEC_OTP_WLOCK0 is used to report permanent write lock of BSEC_OTP_DATA0 to BSEC_OTP_DATA31. BSEC_OTP_WLOCK1 is used to report permanent write lock of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_WLOCK2 is used to report permanent write lock of BSEC_OTP_DATA64 to BSEC_OTP_DATA95. Permanent write lock requires a programming sequence to lock a word (see section:Section3.3.6: OTP operations on page178).
pub mod bsec_otp_wrlock2;
/**BSEC_OTP_SPLOCK0 (rw) register accessor: BSEC_OTP_SPLOCK0 is used to lock the programming of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset BSEC_OTP_SPLOCK1 is used to lock the programming of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset BSEC_OTP_SPLOCK2 is used to lock the programming of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset Attempt to sticky program locked OTP word are silently ignored.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_splock0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_splock0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_SPLOCK0)

For information about available fields see [`mod@bsec_otp_splock0`]
module*/
pub type BSEC_OTP_SPLOCK0 = crate::Reg<bsec_otp_splock0::BSEC_OTP_SPLOCK0rs>;
///BSEC_OTP_SPLOCK0 is used to lock the programming of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset BSEC_OTP_SPLOCK1 is used to lock the programming of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset BSEC_OTP_SPLOCK2 is used to lock the programming of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset Attempt to sticky program locked OTP word are silently ignored.
pub mod bsec_otp_splock0;
/**BSEC_OTP_SPLOCK1 (rw) register accessor: BSEC_OTP_SPLOCK0 is used to lock the programming of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset BSEC_OTP_SPLOCK1 is used to lock the programming of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset BSEC_OTP_SPLOCK2 is used to lock the programming of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset Attempt to sticky program locked OTP word are silently ignored.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_splock1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_splock1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_SPLOCK1)

For information about available fields see [`mod@bsec_otp_splock1`]
module*/
pub type BSEC_OTP_SPLOCK1 = crate::Reg<bsec_otp_splock1::BSEC_OTP_SPLOCK1rs>;
///BSEC_OTP_SPLOCK0 is used to lock the programming of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset BSEC_OTP_SPLOCK1 is used to lock the programming of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset BSEC_OTP_SPLOCK2 is used to lock the programming of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset Attempt to sticky program locked OTP word are silently ignored.
pub mod bsec_otp_splock1;
/**BSEC_OTP_SPLOCK2 (rw) register accessor: BSEC_OTP_SPLOCK0 is used to lock the programming of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset BSEC_OTP_SPLOCK1 is used to lock the programming of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset BSEC_OTP_SPLOCK2 is used to lock the programming of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset Attempt to sticky program locked OTP word are silently ignored.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_splock2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_splock2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_SPLOCK2)

For information about available fields see [`mod@bsec_otp_splock2`]
module*/
pub type BSEC_OTP_SPLOCK2 = crate::Reg<bsec_otp_splock2::BSEC_OTP_SPLOCK2rs>;
///BSEC_OTP_SPLOCK0 is used to lock the programming of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset BSEC_OTP_SPLOCK1 is used to lock the programming of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset BSEC_OTP_SPLOCK2 is used to lock the programming of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset Attempt to sticky program locked OTP word are silently ignored.
pub mod bsec_otp_splock2;
/**BSEC_OTP_SWLOCK0 (rw) register accessor: BSEC_OTP_SWLOCK0 is used to prevent writing to BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SWLOCK1 is used to prevent writing to BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SWLOCK2 is used to prevent writing to BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Write to shadow write locked BSEC_OTP_DATA word are silently ignored. Writing to OTP word 0 shadow is always prevented.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_swlock0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_swlock0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_SWLOCK0)

For information about available fields see [`mod@bsec_otp_swlock0`]
module*/
pub type BSEC_OTP_SWLOCK0 = crate::Reg<bsec_otp_swlock0::BSEC_OTP_SWLOCK0rs>;
///BSEC_OTP_SWLOCK0 is used to prevent writing to BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SWLOCK1 is used to prevent writing to BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SWLOCK2 is used to prevent writing to BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Write to shadow write locked BSEC_OTP_DATA word are silently ignored. Writing to OTP word 0 shadow is always prevented.
pub mod bsec_otp_swlock0;
/**BSEC_OTP_SWLOCK1 (rw) register accessor: BSEC_OTP_SWLOCK0 is used to prevent writing to BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SWLOCK1 is used to prevent writing to BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SWLOCK2 is used to prevent writing to BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Write to shadow write locked BSEC_OTP_DATA word are silently ignored. Writing to OTP word 0 shadow is always prevented.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_swlock1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_swlock1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_SWLOCK1)

For information about available fields see [`mod@bsec_otp_swlock1`]
module*/
pub type BSEC_OTP_SWLOCK1 = crate::Reg<bsec_otp_swlock1::BSEC_OTP_SWLOCK1rs>;
///BSEC_OTP_SWLOCK0 is used to prevent writing to BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SWLOCK1 is used to prevent writing to BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SWLOCK2 is used to prevent writing to BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Write to shadow write locked BSEC_OTP_DATA word are silently ignored. Writing to OTP word 0 shadow is always prevented.
pub mod bsec_otp_swlock1;
/**BSEC_OTP_SWLOCK2 (rw) register accessor: BSEC_OTP_SWLOCK0 is used to prevent writing to BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SWLOCK1 is used to prevent writing to BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SWLOCK2 is used to prevent writing to BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Write to shadow write locked BSEC_OTP_DATA word are silently ignored. Writing to OTP word 0 shadow is always prevented.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_swlock2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_swlock2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_SWLOCK2)

For information about available fields see [`mod@bsec_otp_swlock2`]
module*/
pub type BSEC_OTP_SWLOCK2 = crate::Reg<bsec_otp_swlock2::BSEC_OTP_SWLOCK2rs>;
///BSEC_OTP_SWLOCK0 is used to prevent writing to BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SWLOCK1 is used to prevent writing to BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SWLOCK2 is used to prevent writing to BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Write to shadow write locked BSEC_OTP_DATA word are silently ignored. Writing to OTP word 0 shadow is always prevented.
pub mod bsec_otp_swlock2;
/**BSEC_OTP_SRLOCK0 (rw) register accessor: BSEC_OTP_SRLOCK0 is used to prevent reloading of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SRLOCK1 is used to prevent reloading of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SRLOCK2 is used to prevent reloading of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Setting SRLOCK bits or attempt to reload a locked OTP do not clear the corresponding BSEC_OTP_DATAx shadow register. BSEC_OTP_SRLOCK0 bit 0 is controlled by hardware according to fuse_ok, writing to this bit has no effect.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_srlock0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_srlock0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_SRLOCK0)

For information about available fields see [`mod@bsec_otp_srlock0`]
module*/
pub type BSEC_OTP_SRLOCK0 = crate::Reg<bsec_otp_srlock0::BSEC_OTP_SRLOCK0rs>;
///BSEC_OTP_SRLOCK0 is used to prevent reloading of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SRLOCK1 is used to prevent reloading of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SRLOCK2 is used to prevent reloading of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Setting SRLOCK bits or attempt to reload a locked OTP do not clear the corresponding BSEC_OTP_DATAx shadow register. BSEC_OTP_SRLOCK0 bit 0 is controlled by hardware according to fuse_ok, writing to this bit has no effect.
pub mod bsec_otp_srlock0;
/**BSEC_OTP_SRLOCK1 (rw) register accessor: BSEC_OTP_SRLOCK0 is used to prevent reloading of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SRLOCK1 is used to prevent reloading of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SRLOCK2 is used to prevent reloading of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Setting SRLOCK bits or attempt to reload a locked OTP do not clear the corresponding BSEC_OTP_DATAx shadow register. BSEC_OTP_SRLOCK0 bit 0 is controlled by hardware according to fuse_ok, writing to this bit has no effect.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_srlock1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_srlock1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_SRLOCK1)

For information about available fields see [`mod@bsec_otp_srlock1`]
module*/
pub type BSEC_OTP_SRLOCK1 = crate::Reg<bsec_otp_srlock1::BSEC_OTP_SRLOCK1rs>;
///BSEC_OTP_SRLOCK0 is used to prevent reloading of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SRLOCK1 is used to prevent reloading of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SRLOCK2 is used to prevent reloading of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Setting SRLOCK bits or attempt to reload a locked OTP do not clear the corresponding BSEC_OTP_DATAx shadow register. BSEC_OTP_SRLOCK0 bit 0 is controlled by hardware according to fuse_ok, writing to this bit has no effect.
pub mod bsec_otp_srlock1;
/**BSEC_OTP_SRLOCK2 (rw) register accessor: BSEC_OTP_SRLOCK0 is used to prevent reloading of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SRLOCK1 is used to prevent reloading of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SRLOCK2 is used to prevent reloading of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Setting SRLOCK bits or attempt to reload a locked OTP do not clear the corresponding BSEC_OTP_DATAx shadow register. BSEC_OTP_SRLOCK0 bit 0 is controlled by hardware according to fuse_ok, writing to this bit has no effect.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_srlock2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_srlock2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_SRLOCK2)

For information about available fields see [`mod@bsec_otp_srlock2`]
module*/
pub type BSEC_OTP_SRLOCK2 = crate::Reg<bsec_otp_srlock2::BSEC_OTP_SRLOCK2rs>;
///BSEC_OTP_SRLOCK0 is used to prevent reloading of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SRLOCK1 is used to prevent reloading of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SRLOCK2 is used to prevent reloading of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Setting SRLOCK bits or attempt to reload a locked OTP do not clear the corresponding BSEC_OTP_DATAx shadow register. BSEC_OTP_SRLOCK0 bit 0 is controlled by hardware according to fuse_ok, writing to this bit has no effect.
pub mod bsec_otp_srlock2;
/**BSEC_JTAGIN (r) register accessor: BSEC JTAG input register

You can [`read`](crate::Reg::read) this register and get [`bsec_jtagin::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_JTAGIN)

For information about available fields see [`mod@bsec_jtagin`]
module*/
pub type BSEC_JTAGIN = crate::Reg<bsec_jtagin::BSEC_JTAGINrs>;
///BSEC JTAG input register
pub mod bsec_jtagin;
/**BSEC_JTAGOUT (rw) register accessor: BSEC JTAG output register

You can [`read`](crate::Reg::read) this register and get [`bsec_jtagout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_jtagout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_JTAGOUT)

For information about available fields see [`mod@bsec_jtagout`]
module*/
pub type BSEC_JTAGOUT = crate::Reg<bsec_jtagout::BSEC_JTAGOUTrs>;
///BSEC JTAG output register
pub mod bsec_jtagout;
/**BSEC_SCRATCH (rw) register accessor: BSEC scratch register

You can [`read`](crate::Reg::read) this register and get [`bsec_scratch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_scratch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_SCRATCH)

For information about available fields see [`mod@bsec_scratch`]
module*/
pub type BSEC_SCRATCH = crate::Reg<bsec_scratch::BSEC_SCRATCHrs>;
///BSEC scratch register
pub mod bsec_scratch;
/**BSEC_OTP_DATA0 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA0)

For information about available fields see [`mod@bsec_otp_data0`]
module*/
pub type BSEC_OTP_DATA0 = crate::Reg<bsec_otp_data0::BSEC_OTP_DATA0rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data0;
/**BSEC_OTP_DATA1 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA1)

For information about available fields see [`mod@bsec_otp_data1`]
module*/
pub type BSEC_OTP_DATA1 = crate::Reg<bsec_otp_data1::BSEC_OTP_DATA1rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data1;
/**BSEC_OTP_DATA2 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA2)

For information about available fields see [`mod@bsec_otp_data2`]
module*/
pub type BSEC_OTP_DATA2 = crate::Reg<bsec_otp_data2::BSEC_OTP_DATA2rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data2;
/**BSEC_OTP_DATA3 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA3)

For information about available fields see [`mod@bsec_otp_data3`]
module*/
pub type BSEC_OTP_DATA3 = crate::Reg<bsec_otp_data3::BSEC_OTP_DATA3rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data3;
/**BSEC_OTP_DATA4 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA4)

For information about available fields see [`mod@bsec_otp_data4`]
module*/
pub type BSEC_OTP_DATA4 = crate::Reg<bsec_otp_data4::BSEC_OTP_DATA4rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data4;
/**BSEC_OTP_DATA5 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA5)

For information about available fields see [`mod@bsec_otp_data5`]
module*/
pub type BSEC_OTP_DATA5 = crate::Reg<bsec_otp_data5::BSEC_OTP_DATA5rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data5;
/**BSEC_OTP_DATA6 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA6)

For information about available fields see [`mod@bsec_otp_data6`]
module*/
pub type BSEC_OTP_DATA6 = crate::Reg<bsec_otp_data6::BSEC_OTP_DATA6rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data6;
/**BSEC_OTP_DATA7 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA7)

For information about available fields see [`mod@bsec_otp_data7`]
module*/
pub type BSEC_OTP_DATA7 = crate::Reg<bsec_otp_data7::BSEC_OTP_DATA7rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data7;
/**BSEC_OTP_DATA8 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA8)

For information about available fields see [`mod@bsec_otp_data8`]
module*/
pub type BSEC_OTP_DATA8 = crate::Reg<bsec_otp_data8::BSEC_OTP_DATA8rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data8;
/**BSEC_OTP_DATA9 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA9)

For information about available fields see [`mod@bsec_otp_data9`]
module*/
pub type BSEC_OTP_DATA9 = crate::Reg<bsec_otp_data9::BSEC_OTP_DATA9rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data9;
/**BSEC_OTP_DATA10 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA10)

For information about available fields see [`mod@bsec_otp_data10`]
module*/
pub type BSEC_OTP_DATA10 = crate::Reg<bsec_otp_data10::BSEC_OTP_DATA10rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data10;
/**BSEC_OTP_DATA11 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA11)

For information about available fields see [`mod@bsec_otp_data11`]
module*/
pub type BSEC_OTP_DATA11 = crate::Reg<bsec_otp_data11::BSEC_OTP_DATA11rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data11;
/**BSEC_OTP_DATA12 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA12)

For information about available fields see [`mod@bsec_otp_data12`]
module*/
pub type BSEC_OTP_DATA12 = crate::Reg<bsec_otp_data12::BSEC_OTP_DATA12rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data12;
/**BSEC_OTP_DATA13 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA13)

For information about available fields see [`mod@bsec_otp_data13`]
module*/
pub type BSEC_OTP_DATA13 = crate::Reg<bsec_otp_data13::BSEC_OTP_DATA13rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data13;
/**BSEC_OTP_DATA14 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA14)

For information about available fields see [`mod@bsec_otp_data14`]
module*/
pub type BSEC_OTP_DATA14 = crate::Reg<bsec_otp_data14::BSEC_OTP_DATA14rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data14;
/**BSEC_OTP_DATA15 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA15)

For information about available fields see [`mod@bsec_otp_data15`]
module*/
pub type BSEC_OTP_DATA15 = crate::Reg<bsec_otp_data15::BSEC_OTP_DATA15rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data15;
/**BSEC_OTP_DATA16 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA16)

For information about available fields see [`mod@bsec_otp_data16`]
module*/
pub type BSEC_OTP_DATA16 = crate::Reg<bsec_otp_data16::BSEC_OTP_DATA16rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data16;
/**BSEC_OTP_DATA17 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA17)

For information about available fields see [`mod@bsec_otp_data17`]
module*/
pub type BSEC_OTP_DATA17 = crate::Reg<bsec_otp_data17::BSEC_OTP_DATA17rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data17;
/**BSEC_OTP_DATA18 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA18)

For information about available fields see [`mod@bsec_otp_data18`]
module*/
pub type BSEC_OTP_DATA18 = crate::Reg<bsec_otp_data18::BSEC_OTP_DATA18rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data18;
/**BSEC_OTP_DATA19 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA19)

For information about available fields see [`mod@bsec_otp_data19`]
module*/
pub type BSEC_OTP_DATA19 = crate::Reg<bsec_otp_data19::BSEC_OTP_DATA19rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data19;
/**BSEC_OTP_DATA20 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA20)

For information about available fields see [`mod@bsec_otp_data20`]
module*/
pub type BSEC_OTP_DATA20 = crate::Reg<bsec_otp_data20::BSEC_OTP_DATA20rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data20;
/**BSEC_OTP_DATA21 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA21)

For information about available fields see [`mod@bsec_otp_data21`]
module*/
pub type BSEC_OTP_DATA21 = crate::Reg<bsec_otp_data21::BSEC_OTP_DATA21rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data21;
/**BSEC_OTP_DATA22 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA22)

For information about available fields see [`mod@bsec_otp_data22`]
module*/
pub type BSEC_OTP_DATA22 = crate::Reg<bsec_otp_data22::BSEC_OTP_DATA22rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data22;
/**BSEC_OTP_DATA23 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA23)

For information about available fields see [`mod@bsec_otp_data23`]
module*/
pub type BSEC_OTP_DATA23 = crate::Reg<bsec_otp_data23::BSEC_OTP_DATA23rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data23;
/**BSEC_OTP_DATA24 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA24)

For information about available fields see [`mod@bsec_otp_data24`]
module*/
pub type BSEC_OTP_DATA24 = crate::Reg<bsec_otp_data24::BSEC_OTP_DATA24rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data24;
/**BSEC_OTP_DATA25 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA25)

For information about available fields see [`mod@bsec_otp_data25`]
module*/
pub type BSEC_OTP_DATA25 = crate::Reg<bsec_otp_data25::BSEC_OTP_DATA25rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data25;
/**BSEC_OTP_DATA26 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA26)

For information about available fields see [`mod@bsec_otp_data26`]
module*/
pub type BSEC_OTP_DATA26 = crate::Reg<bsec_otp_data26::BSEC_OTP_DATA26rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data26;
/**BSEC_OTP_DATA27 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA27)

For information about available fields see [`mod@bsec_otp_data27`]
module*/
pub type BSEC_OTP_DATA27 = crate::Reg<bsec_otp_data27::BSEC_OTP_DATA27rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data27;
/**BSEC_OTP_DATA28 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA28)

For information about available fields see [`mod@bsec_otp_data28`]
module*/
pub type BSEC_OTP_DATA28 = crate::Reg<bsec_otp_data28::BSEC_OTP_DATA28rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data28;
/**BSEC_OTP_DATA29 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA29)

For information about available fields see [`mod@bsec_otp_data29`]
module*/
pub type BSEC_OTP_DATA29 = crate::Reg<bsec_otp_data29::BSEC_OTP_DATA29rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data29;
/**BSEC_OTP_DATA30 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA30)

For information about available fields see [`mod@bsec_otp_data30`]
module*/
pub type BSEC_OTP_DATA30 = crate::Reg<bsec_otp_data30::BSEC_OTP_DATA30rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data30;
/**BSEC_OTP_DATA31 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA31)

For information about available fields see [`mod@bsec_otp_data31`]
module*/
pub type BSEC_OTP_DATA31 = crate::Reg<bsec_otp_data31::BSEC_OTP_DATA31rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data31;
/**BSEC_OTP_DATA32 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data32::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data32::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA32)

For information about available fields see [`mod@bsec_otp_data32`]
module*/
pub type BSEC_OTP_DATA32 = crate::Reg<bsec_otp_data32::BSEC_OTP_DATA32rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data32;
/**BSEC_OTP_DATA33 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data33::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data33::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA33)

For information about available fields see [`mod@bsec_otp_data33`]
module*/
pub type BSEC_OTP_DATA33 = crate::Reg<bsec_otp_data33::BSEC_OTP_DATA33rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data33;
/**BSEC_OTP_DATA34 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data34::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data34::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA34)

For information about available fields see [`mod@bsec_otp_data34`]
module*/
pub type BSEC_OTP_DATA34 = crate::Reg<bsec_otp_data34::BSEC_OTP_DATA34rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data34;
/**BSEC_OTP_DATA35 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data35::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data35::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA35)

For information about available fields see [`mod@bsec_otp_data35`]
module*/
pub type BSEC_OTP_DATA35 = crate::Reg<bsec_otp_data35::BSEC_OTP_DATA35rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data35;
/**BSEC_OTP_DATA36 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data36::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data36::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA36)

For information about available fields see [`mod@bsec_otp_data36`]
module*/
pub type BSEC_OTP_DATA36 = crate::Reg<bsec_otp_data36::BSEC_OTP_DATA36rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data36;
/**BSEC_OTP_DATA37 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data37::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data37::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA37)

For information about available fields see [`mod@bsec_otp_data37`]
module*/
pub type BSEC_OTP_DATA37 = crate::Reg<bsec_otp_data37::BSEC_OTP_DATA37rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data37;
/**BSEC_OTP_DATA38 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data38::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data38::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA38)

For information about available fields see [`mod@bsec_otp_data38`]
module*/
pub type BSEC_OTP_DATA38 = crate::Reg<bsec_otp_data38::BSEC_OTP_DATA38rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data38;
/**BSEC_OTP_DATA39 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data39::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data39::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA39)

For information about available fields see [`mod@bsec_otp_data39`]
module*/
pub type BSEC_OTP_DATA39 = crate::Reg<bsec_otp_data39::BSEC_OTP_DATA39rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data39;
/**BSEC_OTP_DATA40 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data40::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data40::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA40)

For information about available fields see [`mod@bsec_otp_data40`]
module*/
pub type BSEC_OTP_DATA40 = crate::Reg<bsec_otp_data40::BSEC_OTP_DATA40rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data40;
/**BSEC_OTP_DATA41 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data41::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data41::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA41)

For information about available fields see [`mod@bsec_otp_data41`]
module*/
pub type BSEC_OTP_DATA41 = crate::Reg<bsec_otp_data41::BSEC_OTP_DATA41rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data41;
/**BSEC_OTP_DATA42 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data42::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data42::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA42)

For information about available fields see [`mod@bsec_otp_data42`]
module*/
pub type BSEC_OTP_DATA42 = crate::Reg<bsec_otp_data42::BSEC_OTP_DATA42rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data42;
/**BSEC_OTP_DATA43 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data43::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data43::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA43)

For information about available fields see [`mod@bsec_otp_data43`]
module*/
pub type BSEC_OTP_DATA43 = crate::Reg<bsec_otp_data43::BSEC_OTP_DATA43rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data43;
/**BSEC_OTP_DATA44 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data44::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data44::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA44)

For information about available fields see [`mod@bsec_otp_data44`]
module*/
pub type BSEC_OTP_DATA44 = crate::Reg<bsec_otp_data44::BSEC_OTP_DATA44rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data44;
/**BSEC_OTP_DATA45 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data45::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data45::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA45)

For information about available fields see [`mod@bsec_otp_data45`]
module*/
pub type BSEC_OTP_DATA45 = crate::Reg<bsec_otp_data45::BSEC_OTP_DATA45rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data45;
/**BSEC_OTP_DATA46 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data46::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data46::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA46)

For information about available fields see [`mod@bsec_otp_data46`]
module*/
pub type BSEC_OTP_DATA46 = crate::Reg<bsec_otp_data46::BSEC_OTP_DATA46rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data46;
/**BSEC_OTP_DATA47 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data47::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data47::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA47)

For information about available fields see [`mod@bsec_otp_data47`]
module*/
pub type BSEC_OTP_DATA47 = crate::Reg<bsec_otp_data47::BSEC_OTP_DATA47rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data47;
/**BSEC_OTP_DATA48 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data48::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data48::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA48)

For information about available fields see [`mod@bsec_otp_data48`]
module*/
pub type BSEC_OTP_DATA48 = crate::Reg<bsec_otp_data48::BSEC_OTP_DATA48rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data48;
/**BSEC_OTP_DATA49 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data49::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data49::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA49)

For information about available fields see [`mod@bsec_otp_data49`]
module*/
pub type BSEC_OTP_DATA49 = crate::Reg<bsec_otp_data49::BSEC_OTP_DATA49rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data49;
/**BSEC_OTP_DATA50 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data50::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data50::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA50)

For information about available fields see [`mod@bsec_otp_data50`]
module*/
pub type BSEC_OTP_DATA50 = crate::Reg<bsec_otp_data50::BSEC_OTP_DATA50rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data50;
/**BSEC_OTP_DATA51 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data51::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data51::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA51)

For information about available fields see [`mod@bsec_otp_data51`]
module*/
pub type BSEC_OTP_DATA51 = crate::Reg<bsec_otp_data51::BSEC_OTP_DATA51rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data51;
/**BSEC_OTP_DATA52 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data52::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data52::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA52)

For information about available fields see [`mod@bsec_otp_data52`]
module*/
pub type BSEC_OTP_DATA52 = crate::Reg<bsec_otp_data52::BSEC_OTP_DATA52rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data52;
/**BSEC_OTP_DATA53 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data53::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data53::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA53)

For information about available fields see [`mod@bsec_otp_data53`]
module*/
pub type BSEC_OTP_DATA53 = crate::Reg<bsec_otp_data53::BSEC_OTP_DATA53rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data53;
/**BSEC_OTP_DATA54 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data54::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data54::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA54)

For information about available fields see [`mod@bsec_otp_data54`]
module*/
pub type BSEC_OTP_DATA54 = crate::Reg<bsec_otp_data54::BSEC_OTP_DATA54rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data54;
/**BSEC_OTP_DATA55 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data55::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data55::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA55)

For information about available fields see [`mod@bsec_otp_data55`]
module*/
pub type BSEC_OTP_DATA55 = crate::Reg<bsec_otp_data55::BSEC_OTP_DATA55rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data55;
/**BSEC_OTP_DATA56 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data56::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data56::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA56)

For information about available fields see [`mod@bsec_otp_data56`]
module*/
pub type BSEC_OTP_DATA56 = crate::Reg<bsec_otp_data56::BSEC_OTP_DATA56rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data56;
/**BSEC_OTP_DATA57 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data57::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data57::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA57)

For information about available fields see [`mod@bsec_otp_data57`]
module*/
pub type BSEC_OTP_DATA57 = crate::Reg<bsec_otp_data57::BSEC_OTP_DATA57rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data57;
/**BSEC_OTP_DATA58 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data58::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data58::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA58)

For information about available fields see [`mod@bsec_otp_data58`]
module*/
pub type BSEC_OTP_DATA58 = crate::Reg<bsec_otp_data58::BSEC_OTP_DATA58rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data58;
/**BSEC_OTP_DATA59 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data59::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data59::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA59)

For information about available fields see [`mod@bsec_otp_data59`]
module*/
pub type BSEC_OTP_DATA59 = crate::Reg<bsec_otp_data59::BSEC_OTP_DATA59rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data59;
/**BSEC_OTP_DATA60 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data60::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data60::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA60)

For information about available fields see [`mod@bsec_otp_data60`]
module*/
pub type BSEC_OTP_DATA60 = crate::Reg<bsec_otp_data60::BSEC_OTP_DATA60rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data60;
/**BSEC_OTP_DATA61 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data61::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data61::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA61)

For information about available fields see [`mod@bsec_otp_data61`]
module*/
pub type BSEC_OTP_DATA61 = crate::Reg<bsec_otp_data61::BSEC_OTP_DATA61rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data61;
/**BSEC_OTP_DATA62 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data62::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data62::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA62)

For information about available fields see [`mod@bsec_otp_data62`]
module*/
pub type BSEC_OTP_DATA62 = crate::Reg<bsec_otp_data62::BSEC_OTP_DATA62rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data62;
/**BSEC_OTP_DATA63 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data63::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data63::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA63)

For information about available fields see [`mod@bsec_otp_data63`]
module*/
pub type BSEC_OTP_DATA63 = crate::Reg<bsec_otp_data63::BSEC_OTP_DATA63rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data63;
/**BSEC_OTP_DATA64 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data64::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data64::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA64)

For information about available fields see [`mod@bsec_otp_data64`]
module*/
pub type BSEC_OTP_DATA64 = crate::Reg<bsec_otp_data64::BSEC_OTP_DATA64rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data64;
/**BSEC_OTP_DATA65 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data65::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data65::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA65)

For information about available fields see [`mod@bsec_otp_data65`]
module*/
pub type BSEC_OTP_DATA65 = crate::Reg<bsec_otp_data65::BSEC_OTP_DATA65rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data65;
/**BSEC_OTP_DATA66 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data66::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data66::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA66)

For information about available fields see [`mod@bsec_otp_data66`]
module*/
pub type BSEC_OTP_DATA66 = crate::Reg<bsec_otp_data66::BSEC_OTP_DATA66rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data66;
/**BSEC_OTP_DATA67 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data67::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data67::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA67)

For information about available fields see [`mod@bsec_otp_data67`]
module*/
pub type BSEC_OTP_DATA67 = crate::Reg<bsec_otp_data67::BSEC_OTP_DATA67rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data67;
/**BSEC_OTP_DATA68 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data68::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data68::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA68)

For information about available fields see [`mod@bsec_otp_data68`]
module*/
pub type BSEC_OTP_DATA68 = crate::Reg<bsec_otp_data68::BSEC_OTP_DATA68rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data68;
/**BSEC_OTP_DATA69 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data69::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data69::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA69)

For information about available fields see [`mod@bsec_otp_data69`]
module*/
pub type BSEC_OTP_DATA69 = crate::Reg<bsec_otp_data69::BSEC_OTP_DATA69rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data69;
/**BSEC_OTP_DATA70 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data70::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data70::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA70)

For information about available fields see [`mod@bsec_otp_data70`]
module*/
pub type BSEC_OTP_DATA70 = crate::Reg<bsec_otp_data70::BSEC_OTP_DATA70rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data70;
/**BSEC_OTP_DATA71 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data71::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data71::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA71)

For information about available fields see [`mod@bsec_otp_data71`]
module*/
pub type BSEC_OTP_DATA71 = crate::Reg<bsec_otp_data71::BSEC_OTP_DATA71rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data71;
/**BSEC_OTP_DATA72 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data72::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data72::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA72)

For information about available fields see [`mod@bsec_otp_data72`]
module*/
pub type BSEC_OTP_DATA72 = crate::Reg<bsec_otp_data72::BSEC_OTP_DATA72rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data72;
/**BSEC_OTP_DATA73 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data73::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data73::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA73)

For information about available fields see [`mod@bsec_otp_data73`]
module*/
pub type BSEC_OTP_DATA73 = crate::Reg<bsec_otp_data73::BSEC_OTP_DATA73rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data73;
/**BSEC_OTP_DATA74 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data74::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data74::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA74)

For information about available fields see [`mod@bsec_otp_data74`]
module*/
pub type BSEC_OTP_DATA74 = crate::Reg<bsec_otp_data74::BSEC_OTP_DATA74rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data74;
/**BSEC_OTP_DATA75 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data75::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data75::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA75)

For information about available fields see [`mod@bsec_otp_data75`]
module*/
pub type BSEC_OTP_DATA75 = crate::Reg<bsec_otp_data75::BSEC_OTP_DATA75rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data75;
/**BSEC_OTP_DATA76 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data76::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data76::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA76)

For information about available fields see [`mod@bsec_otp_data76`]
module*/
pub type BSEC_OTP_DATA76 = crate::Reg<bsec_otp_data76::BSEC_OTP_DATA76rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data76;
/**BSEC_OTP_DATA77 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data77::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data77::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA77)

For information about available fields see [`mod@bsec_otp_data77`]
module*/
pub type BSEC_OTP_DATA77 = crate::Reg<bsec_otp_data77::BSEC_OTP_DATA77rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data77;
/**BSEC_OTP_DATA78 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data78::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data78::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA78)

For information about available fields see [`mod@bsec_otp_data78`]
module*/
pub type BSEC_OTP_DATA78 = crate::Reg<bsec_otp_data78::BSEC_OTP_DATA78rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data78;
/**BSEC_OTP_DATA79 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data79::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data79::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA79)

For information about available fields see [`mod@bsec_otp_data79`]
module*/
pub type BSEC_OTP_DATA79 = crate::Reg<bsec_otp_data79::BSEC_OTP_DATA79rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data79;
/**BSEC_OTP_DATA80 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data80::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data80::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA80)

For information about available fields see [`mod@bsec_otp_data80`]
module*/
pub type BSEC_OTP_DATA80 = crate::Reg<bsec_otp_data80::BSEC_OTP_DATA80rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data80;
/**BSEC_OTP_DATA81 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data81::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data81::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA81)

For information about available fields see [`mod@bsec_otp_data81`]
module*/
pub type BSEC_OTP_DATA81 = crate::Reg<bsec_otp_data81::BSEC_OTP_DATA81rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data81;
/**BSEC_OTP_DATA82 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data82::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data82::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA82)

For information about available fields see [`mod@bsec_otp_data82`]
module*/
pub type BSEC_OTP_DATA82 = crate::Reg<bsec_otp_data82::BSEC_OTP_DATA82rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data82;
/**BSEC_OTP_DATA83 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data83::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data83::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA83)

For information about available fields see [`mod@bsec_otp_data83`]
module*/
pub type BSEC_OTP_DATA83 = crate::Reg<bsec_otp_data83::BSEC_OTP_DATA83rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data83;
/**BSEC_OTP_DATA84 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data84::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data84::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA84)

For information about available fields see [`mod@bsec_otp_data84`]
module*/
pub type BSEC_OTP_DATA84 = crate::Reg<bsec_otp_data84::BSEC_OTP_DATA84rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data84;
/**BSEC_OTP_DATA85 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data85::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data85::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA85)

For information about available fields see [`mod@bsec_otp_data85`]
module*/
pub type BSEC_OTP_DATA85 = crate::Reg<bsec_otp_data85::BSEC_OTP_DATA85rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data85;
/**BSEC_OTP_DATA86 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data86::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data86::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA86)

For information about available fields see [`mod@bsec_otp_data86`]
module*/
pub type BSEC_OTP_DATA86 = crate::Reg<bsec_otp_data86::BSEC_OTP_DATA86rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data86;
/**BSEC_OTP_DATA87 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data87::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data87::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA87)

For information about available fields see [`mod@bsec_otp_data87`]
module*/
pub type BSEC_OTP_DATA87 = crate::Reg<bsec_otp_data87::BSEC_OTP_DATA87rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data87;
/**BSEC_OTP_DATA88 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data88::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data88::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA88)

For information about available fields see [`mod@bsec_otp_data88`]
module*/
pub type BSEC_OTP_DATA88 = crate::Reg<bsec_otp_data88::BSEC_OTP_DATA88rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data88;
/**BSEC_OTP_DATA89 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data89::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data89::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA89)

For information about available fields see [`mod@bsec_otp_data89`]
module*/
pub type BSEC_OTP_DATA89 = crate::Reg<bsec_otp_data89::BSEC_OTP_DATA89rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data89;
/**BSEC_OTP_DATA90 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data90::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data90::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA90)

For information about available fields see [`mod@bsec_otp_data90`]
module*/
pub type BSEC_OTP_DATA90 = crate::Reg<bsec_otp_data90::BSEC_OTP_DATA90rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data90;
/**BSEC_OTP_DATA91 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data91::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data91::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA91)

For information about available fields see [`mod@bsec_otp_data91`]
module*/
pub type BSEC_OTP_DATA91 = crate::Reg<bsec_otp_data91::BSEC_OTP_DATA91rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data91;
/**BSEC_OTP_DATA92 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data92::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data92::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA92)

For information about available fields see [`mod@bsec_otp_data92`]
module*/
pub type BSEC_OTP_DATA92 = crate::Reg<bsec_otp_data92::BSEC_OTP_DATA92rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data92;
/**BSEC_OTP_DATA93 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data93::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data93::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA93)

For information about available fields see [`mod@bsec_otp_data93`]
module*/
pub type BSEC_OTP_DATA93 = crate::Reg<bsec_otp_data93::BSEC_OTP_DATA93rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data93;
/**BSEC_OTP_DATA94 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data94::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data94::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA94)

For information about available fields see [`mod@bsec_otp_data94`]
module*/
pub type BSEC_OTP_DATA94 = crate::Reg<bsec_otp_data94::BSEC_OTP_DATA94rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data94;
/**BSEC_OTP_DATA95 (rw) register accessor: Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data95::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data95::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_DATA95)

For information about available fields see [`mod@bsec_otp_data95`]
module*/
pub type BSEC_OTP_DATA95 = crate::Reg<bsec_otp_data95::BSEC_OTP_DATA95rs>;
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.*/
pub mod bsec_otp_data95;
/**BSEC_HWCFGR (r) register accessor: BSEC hardware configuration register

You can [`read`](crate::Reg::read) this register and get [`bsec_hwcfgr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_HWCFGR)

For information about available fields see [`mod@bsec_hwcfgr`]
module*/
pub type BSEC_HWCFGR = crate::Reg<bsec_hwcfgr::BSEC_HWCFGRrs>;
///BSEC hardware configuration register
pub mod bsec_hwcfgr;
/**BSEC_VERR (r) register accessor: BSEC version register

You can [`read`](crate::Reg::read) this register and get [`bsec_verr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_VERR)

For information about available fields see [`mod@bsec_verr`]
module*/
pub type BSEC_VERR = crate::Reg<bsec_verr::BSEC_VERRrs>;
///BSEC version register
pub mod bsec_verr;
/**BSEC_IPIDR (r) register accessor: BSEC identification register

You can [`read`](crate::Reg::read) this register and get [`bsec_ipidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_IPIDR)

For information about available fields see [`mod@bsec_ipidr`]
module*/
pub type BSEC_IPIDR = crate::Reg<bsec_ipidr::BSEC_IPIDRrs>;
///BSEC identification register
pub mod bsec_ipidr;
/**BSEC_SIDR (r) register accessor: BSEC size identification register

You can [`read`](crate::Reg::read) this register and get [`bsec_sidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_SIDR)

For information about available fields see [`mod@bsec_sidr`]
module*/
pub type BSEC_SIDR = crate::Reg<bsec_sidr::BSEC_SIDRrs>;
///BSEC size identification register
pub mod bsec_sidr;
