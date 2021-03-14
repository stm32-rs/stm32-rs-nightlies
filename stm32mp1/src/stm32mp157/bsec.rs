#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - BSEC OTP configuration register"]
    pub bsec_otp_config: BSEC_OTP_CONFIG,
    #[doc = "0x04 - BSEC OTP control register"]
    pub bsec_otp_control: BSEC_OTP_CONTROL,
    #[doc = "0x08 - BSEC OTP write data register"]
    pub bsec_otp_wrdata: BSEC_OTP_WRDATA,
    #[doc = "0x0c - BSEC OTP status register"]
    pub bsec_otp_status: BSEC_OTP_STATUS,
    #[doc = "0x10 - BSEC OTP lock configuration register"]
    pub bsec_otp_lock: BSEC_OTP_LOCK,
    #[doc = "0x14 - reset value depends on OTP secure mode according toTable18: BSEC_DENABLE default values after reset on page181."]
    pub bsec_denable: BSEC_DENABLE,
    _reserved6: [u8; 4usize],
    #[doc = "0x1c - BSEC_OTP_DISTURBED0 is used to report disturbed state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP). BSEC_OTP_DISTURBED1 is used to report disturbed state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_DISTURBED2 is used to report disturbed state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95."]
    pub bsec_otp_disturbed0: BSEC_OTP_DISTURBED0,
    #[doc = "0x20 - BSEC_OTP_DISTURBED0 is used to report disturbed state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP). BSEC_OTP_DISTURBED1 is used to report disturbed state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_DISTURBED2 is used to report disturbed state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95."]
    pub bsec_otp_disturbed1: BSEC_OTP_DISTURBED1,
    #[doc = "0x24 - BSEC_OTP_DISTURBED0 is used to report disturbed state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP). BSEC_OTP_DISTURBED1 is used to report disturbed state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_DISTURBED2 is used to report disturbed state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95."]
    pub bsec_otp_disturbed2: BSEC_OTP_DISTURBED2,
    _reserved9: [u8; 12usize],
    #[doc = "0x34 - BSEC_OTP_ERROR0 is used to report error state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP which are protected by 2:1 redundancy). BSEC_OTP_ERROR1 is used to report error state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 which are protected by 6-bit ECC. BSEC_OTP_ERROR2 is used to report error state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 which are protected by 6-bit ECC."]
    pub bsec_otp_error0: BSEC_OTP_ERROR0,
    #[doc = "0x38 - BSEC_OTP_ERROR0 is used to report error state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP which are protected by 2:1 redundancy). BSEC_OTP_ERROR1 is used to report error state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 which are protected by 6-bit ECC. BSEC_OTP_ERROR2 is used to report error state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 which are protected by 6-bit ECC."]
    pub bsec_otp_error1: BSEC_OTP_ERROR1,
    #[doc = "0x3c - BSEC_OTP_ERROR0 is used to report error state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP which are protected by 2:1 redundancy). BSEC_OTP_ERROR1 is used to report error state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 which are protected by 6-bit ECC. BSEC_OTP_ERROR2 is used to report error state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 which are protected by 6-bit ECC."]
    pub bsec_otp_error2: BSEC_OTP_ERROR2,
    _reserved12: [u8; 12usize],
    #[doc = "0x4c - BSEC_OTP_WLOCK0 is used to report permanent write lock of BSEC_OTP_DATA0 to BSEC_OTP_DATA31. BSEC_OTP_WLOCK1 is used to report permanent write lock of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_WLOCK2 is used to report permanent write lock of BSEC_OTP_DATA64 to BSEC_OTP_DATA95. Permanent write lock requires a programming sequence to lock a word (see section:Section3.3.6: OTP operations on page178)."]
    pub bsec_otp_wrlock0: BSEC_OTP_WRLOCK0,
    #[doc = "0x50 - BSEC_OTP_WLOCK0 is used to report permanent write lock of BSEC_OTP_DATA0 to BSEC_OTP_DATA31. BSEC_OTP_WLOCK1 is used to report permanent write lock of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_WLOCK2 is used to report permanent write lock of BSEC_OTP_DATA64 to BSEC_OTP_DATA95. Permanent write lock requires a programming sequence to lock a word (see section:Section3.3.6: OTP operations on page178)."]
    pub bsec_otp_wrlock1: BSEC_OTP_WRLOCK1,
    #[doc = "0x54 - BSEC_OTP_WLOCK0 is used to report permanent write lock of BSEC_OTP_DATA0 to BSEC_OTP_DATA31. BSEC_OTP_WLOCK1 is used to report permanent write lock of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_WLOCK2 is used to report permanent write lock of BSEC_OTP_DATA64 to BSEC_OTP_DATA95. Permanent write lock requires a programming sequence to lock a word (see section:Section3.3.6: OTP operations on page178)."]
    pub bsec_otp_wrlock2: BSEC_OTP_WRLOCK2,
    _reserved15: [u8; 12usize],
    #[doc = "0x64 - BSEC_OTP_SPLOCK0 is used to lock the programming of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset BSEC_OTP_SPLOCK1 is used to lock the programming of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset BSEC_OTP_SPLOCK2 is used to lock the programming of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset Attempt to sticky program locked OTP word are silently ignored."]
    pub bsec_otp_splock0: BSEC_OTP_SPLOCK0,
    #[doc = "0x68 - BSEC_OTP_SPLOCK0 is used to lock the programming of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset BSEC_OTP_SPLOCK1 is used to lock the programming of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset BSEC_OTP_SPLOCK2 is used to lock the programming of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset Attempt to sticky program locked OTP word are silently ignored."]
    pub bsec_otp_splock1: BSEC_OTP_SPLOCK1,
    #[doc = "0x6c - BSEC_OTP_SPLOCK0 is used to lock the programming of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset BSEC_OTP_SPLOCK1 is used to lock the programming of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset BSEC_OTP_SPLOCK2 is used to lock the programming of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset Attempt to sticky program locked OTP word are silently ignored."]
    pub bsec_otp_splock2: BSEC_OTP_SPLOCK2,
    _reserved18: [u8; 12usize],
    #[doc = "0x7c - BSEC_OTP_SWLOCK0 is used to prevent writing to BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SWLOCK1 is used to prevent writing to BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SWLOCK2 is used to prevent writing to BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Write to shadow write locked BSEC_OTP_DATA word are silently ignored. Writing to OTP word 0 shadow is always prevented."]
    pub bsec_otp_swlock0: BSEC_OTP_SWLOCK0,
    #[doc = "0x80 - BSEC_OTP_SWLOCK0 is used to prevent writing to BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SWLOCK1 is used to prevent writing to BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SWLOCK2 is used to prevent writing to BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Write to shadow write locked BSEC_OTP_DATA word are silently ignored. Writing to OTP word 0 shadow is always prevented."]
    pub bsec_otp_swlock1: BSEC_OTP_SWLOCK1,
    #[doc = "0x84 - BSEC_OTP_SWLOCK0 is used to prevent writing to BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SWLOCK1 is used to prevent writing to BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SWLOCK2 is used to prevent writing to BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Write to shadow write locked BSEC_OTP_DATA word are silently ignored. Writing to OTP word 0 shadow is always prevented."]
    pub bsec_otp_swlock2: BSEC_OTP_SWLOCK2,
    _reserved21: [u8; 12usize],
    #[doc = "0x94 - BSEC_OTP_SRLOCK0 is used to prevent reloading of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SRLOCK1 is used to prevent reloading of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SRLOCK2 is used to prevent reloading of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Setting SRLOCK bits or attempt to reload a locked OTP do not clear the corresponding BSEC_OTP_DATAx shadow register. BSEC_OTP_SRLOCK0 bit 0 is controlled by hardware according to fuse_ok, writing to this bit has no effect."]
    pub bsec_otp_srlock0: BSEC_OTP_SRLOCK0,
    #[doc = "0x98 - BSEC_OTP_SRLOCK0 is used to prevent reloading of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SRLOCK1 is used to prevent reloading of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SRLOCK2 is used to prevent reloading of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Setting SRLOCK bits or attempt to reload a locked OTP do not clear the corresponding BSEC_OTP_DATAx shadow register. BSEC_OTP_SRLOCK0 bit 0 is controlled by hardware according to fuse_ok, writing to this bit has no effect."]
    pub bsec_otp_srlock1: BSEC_OTP_SRLOCK1,
    #[doc = "0x9c - BSEC_OTP_SRLOCK0 is used to prevent reloading of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SRLOCK1 is used to prevent reloading of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SRLOCK2 is used to prevent reloading of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Setting SRLOCK bits or attempt to reload a locked OTP do not clear the corresponding BSEC_OTP_DATAx shadow register. BSEC_OTP_SRLOCK0 bit 0 is controlled by hardware according to fuse_ok, writing to this bit has no effect."]
    pub bsec_otp_srlock2: BSEC_OTP_SRLOCK2,
    _reserved24: [u8; 12usize],
    #[doc = "0xac - BSEC JTAG input register"]
    pub bsec_jtagin: BSEC_JTAGIN,
    #[doc = "0xb0 - BSEC JTAG output register"]
    pub bsec_jtagout: BSEC_JTAGOUT,
    #[doc = "0xb4 - BSEC scratch register"]
    pub bsec_scratch: BSEC_SCRATCH,
    _reserved27: [u8; 328usize],
    #[doc = "0x200 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data0: BSEC_OTP_DATA0,
    #[doc = "0x204 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data1: BSEC_OTP_DATA1,
    #[doc = "0x208 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data2: BSEC_OTP_DATA2,
    #[doc = "0x20c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data3: BSEC_OTP_DATA3,
    #[doc = "0x210 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data4: BSEC_OTP_DATA4,
    #[doc = "0x214 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data5: BSEC_OTP_DATA5,
    #[doc = "0x218 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data6: BSEC_OTP_DATA6,
    #[doc = "0x21c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data7: BSEC_OTP_DATA7,
    #[doc = "0x220 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data8: BSEC_OTP_DATA8,
    #[doc = "0x224 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data9: BSEC_OTP_DATA9,
    #[doc = "0x228 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data10: BSEC_OTP_DATA10,
    #[doc = "0x22c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data11: BSEC_OTP_DATA11,
    #[doc = "0x230 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data12: BSEC_OTP_DATA12,
    #[doc = "0x234 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data13: BSEC_OTP_DATA13,
    #[doc = "0x238 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data14: BSEC_OTP_DATA14,
    #[doc = "0x23c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data15: BSEC_OTP_DATA15,
    #[doc = "0x240 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data16: BSEC_OTP_DATA16,
    #[doc = "0x244 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data17: BSEC_OTP_DATA17,
    #[doc = "0x248 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data18: BSEC_OTP_DATA18,
    #[doc = "0x24c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data19: BSEC_OTP_DATA19,
    #[doc = "0x250 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data20: BSEC_OTP_DATA20,
    #[doc = "0x254 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data21: BSEC_OTP_DATA21,
    #[doc = "0x258 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data22: BSEC_OTP_DATA22,
    #[doc = "0x25c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data23: BSEC_OTP_DATA23,
    #[doc = "0x260 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data24: BSEC_OTP_DATA24,
    #[doc = "0x264 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data25: BSEC_OTP_DATA25,
    #[doc = "0x268 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data26: BSEC_OTP_DATA26,
    #[doc = "0x26c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data27: BSEC_OTP_DATA27,
    #[doc = "0x270 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data28: BSEC_OTP_DATA28,
    #[doc = "0x274 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data29: BSEC_OTP_DATA29,
    #[doc = "0x278 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data30: BSEC_OTP_DATA30,
    #[doc = "0x27c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data31: BSEC_OTP_DATA31,
    #[doc = "0x280 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data32: BSEC_OTP_DATA32,
    #[doc = "0x284 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data33: BSEC_OTP_DATA33,
    #[doc = "0x288 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data34: BSEC_OTP_DATA34,
    #[doc = "0x28c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data35: BSEC_OTP_DATA35,
    #[doc = "0x290 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data36: BSEC_OTP_DATA36,
    #[doc = "0x294 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data37: BSEC_OTP_DATA37,
    #[doc = "0x298 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data38: BSEC_OTP_DATA38,
    #[doc = "0x29c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data39: BSEC_OTP_DATA39,
    #[doc = "0x2a0 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data40: BSEC_OTP_DATA40,
    #[doc = "0x2a4 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data41: BSEC_OTP_DATA41,
    #[doc = "0x2a8 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data42: BSEC_OTP_DATA42,
    #[doc = "0x2ac - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data43: BSEC_OTP_DATA43,
    #[doc = "0x2b0 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data44: BSEC_OTP_DATA44,
    #[doc = "0x2b4 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data45: BSEC_OTP_DATA45,
    #[doc = "0x2b8 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data46: BSEC_OTP_DATA46,
    #[doc = "0x2bc - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data47: BSEC_OTP_DATA47,
    #[doc = "0x2c0 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data48: BSEC_OTP_DATA48,
    #[doc = "0x2c4 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data49: BSEC_OTP_DATA49,
    #[doc = "0x2c8 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data50: BSEC_OTP_DATA50,
    #[doc = "0x2cc - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data51: BSEC_OTP_DATA51,
    #[doc = "0x2d0 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data52: BSEC_OTP_DATA52,
    #[doc = "0x2d4 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data53: BSEC_OTP_DATA53,
    #[doc = "0x2d8 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data54: BSEC_OTP_DATA54,
    #[doc = "0x2dc - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data55: BSEC_OTP_DATA55,
    #[doc = "0x2e0 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data56: BSEC_OTP_DATA56,
    #[doc = "0x2e4 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data57: BSEC_OTP_DATA57,
    #[doc = "0x2e8 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data58: BSEC_OTP_DATA58,
    #[doc = "0x2ec - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data59: BSEC_OTP_DATA59,
    #[doc = "0x2f0 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data60: BSEC_OTP_DATA60,
    #[doc = "0x2f4 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data61: BSEC_OTP_DATA61,
    #[doc = "0x2f8 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data62: BSEC_OTP_DATA62,
    #[doc = "0x2fc - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data63: BSEC_OTP_DATA63,
    #[doc = "0x300 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data64: BSEC_OTP_DATA64,
    #[doc = "0x304 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data65: BSEC_OTP_DATA65,
    #[doc = "0x308 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data66: BSEC_OTP_DATA66,
    #[doc = "0x30c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data67: BSEC_OTP_DATA67,
    #[doc = "0x310 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data68: BSEC_OTP_DATA68,
    #[doc = "0x314 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data69: BSEC_OTP_DATA69,
    #[doc = "0x318 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data70: BSEC_OTP_DATA70,
    #[doc = "0x31c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data71: BSEC_OTP_DATA71,
    #[doc = "0x320 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data72: BSEC_OTP_DATA72,
    #[doc = "0x324 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data73: BSEC_OTP_DATA73,
    #[doc = "0x328 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data74: BSEC_OTP_DATA74,
    #[doc = "0x32c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data75: BSEC_OTP_DATA75,
    #[doc = "0x330 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data76: BSEC_OTP_DATA76,
    #[doc = "0x334 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data77: BSEC_OTP_DATA77,
    #[doc = "0x338 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data78: BSEC_OTP_DATA78,
    #[doc = "0x33c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data79: BSEC_OTP_DATA79,
    #[doc = "0x340 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data80: BSEC_OTP_DATA80,
    #[doc = "0x344 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data81: BSEC_OTP_DATA81,
    #[doc = "0x348 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data82: BSEC_OTP_DATA82,
    #[doc = "0x34c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data83: BSEC_OTP_DATA83,
    #[doc = "0x350 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data84: BSEC_OTP_DATA84,
    #[doc = "0x354 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data85: BSEC_OTP_DATA85,
    #[doc = "0x358 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data86: BSEC_OTP_DATA86,
    #[doc = "0x35c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data87: BSEC_OTP_DATA87,
    #[doc = "0x360 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data88: BSEC_OTP_DATA88,
    #[doc = "0x364 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data89: BSEC_OTP_DATA89,
    #[doc = "0x368 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data90: BSEC_OTP_DATA90,
    #[doc = "0x36c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data91: BSEC_OTP_DATA91,
    #[doc = "0x370 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data92: BSEC_OTP_DATA92,
    #[doc = "0x374 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data93: BSEC_OTP_DATA93,
    #[doc = "0x378 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data94: BSEC_OTP_DATA94,
    #[doc = "0x37c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
    pub bsec_otp_data95: BSEC_OTP_DATA95,
    _reserved123: [u8; 3184usize],
    #[doc = "0xff0 - BSEC hardware configuration register"]
    pub bsec_hwcfgr: BSEC_HWCFGR,
    #[doc = "0xff4 - BSEC version register"]
    pub bsec_verr: BSEC_VERR,
    #[doc = "0xff8 - BSEC identification register"]
    pub bsec_ipidr: BSEC_IPIDR,
    #[doc = "0xffc - BSEC size identification register"]
    pub bsec_sidr: BSEC_SIDR,
}
#[doc = "BSEC OTP configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_config](bsec_otp_config) module"]
pub type BSEC_OTP_CONFIG = crate::Reg<u32, _BSEC_OTP_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_CONFIG;
#[doc = "`read()` method returns [bsec_otp_config::R](bsec_otp_config::R) reader structure"]
impl crate::Readable for BSEC_OTP_CONFIG {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_config::W](bsec_otp_config::W) writer structure"]
impl crate::Writable for BSEC_OTP_CONFIG {}
#[doc = "BSEC OTP configuration register"]
pub mod bsec_otp_config;
#[doc = "BSEC OTP control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_control](bsec_otp_control) module"]
pub type BSEC_OTP_CONTROL = crate::Reg<u32, _BSEC_OTP_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_CONTROL;
#[doc = "`read()` method returns [bsec_otp_control::R](bsec_otp_control::R) reader structure"]
impl crate::Readable for BSEC_OTP_CONTROL {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_control::W](bsec_otp_control::W) writer structure"]
impl crate::Writable for BSEC_OTP_CONTROL {}
#[doc = "BSEC OTP control register"]
pub mod bsec_otp_control;
#[doc = "BSEC OTP write data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_wrdata](bsec_otp_wrdata) module"]
pub type BSEC_OTP_WRDATA = crate::Reg<u32, _BSEC_OTP_WRDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_WRDATA;
#[doc = "`read()` method returns [bsec_otp_wrdata::R](bsec_otp_wrdata::R) reader structure"]
impl crate::Readable for BSEC_OTP_WRDATA {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_wrdata::W](bsec_otp_wrdata::W) writer structure"]
impl crate::Writable for BSEC_OTP_WRDATA {}
#[doc = "BSEC OTP write data register"]
pub mod bsec_otp_wrdata;
#[doc = "BSEC OTP status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_status](bsec_otp_status) module"]
pub type BSEC_OTP_STATUS = crate::Reg<u32, _BSEC_OTP_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_STATUS;
#[doc = "`read()` method returns [bsec_otp_status::R](bsec_otp_status::R) reader structure"]
impl crate::Readable for BSEC_OTP_STATUS {}
#[doc = "BSEC OTP status register"]
pub mod bsec_otp_status;
#[doc = "BSEC OTP lock configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_lock](bsec_otp_lock) module"]
pub type BSEC_OTP_LOCK = crate::Reg<u32, _BSEC_OTP_LOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_LOCK;
#[doc = "`read()` method returns [bsec_otp_lock::R](bsec_otp_lock::R) reader structure"]
impl crate::Readable for BSEC_OTP_LOCK {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_lock::W](bsec_otp_lock::W) writer structure"]
impl crate::Writable for BSEC_OTP_LOCK {}
#[doc = "BSEC OTP lock configuration register"]
pub mod bsec_otp_lock;
#[doc = "reset value depends on OTP secure mode according toTable18: BSEC_DENABLE default values after reset on page181.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_denable](bsec_denable) module"]
pub type BSEC_DENABLE = crate::Reg<u32, _BSEC_DENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_DENABLE;
#[doc = "`read()` method returns [bsec_denable::R](bsec_denable::R) reader structure"]
impl crate::Readable for BSEC_DENABLE {}
#[doc = "`write(|w| ..)` method takes [bsec_denable::W](bsec_denable::W) writer structure"]
impl crate::Writable for BSEC_DENABLE {}
#[doc = "reset value depends on OTP secure mode according toTable18: BSEC_DENABLE default values after reset on page181."]
pub mod bsec_denable;
#[doc = "BSEC_OTP_DISTURBED0 is used to report disturbed state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP). BSEC_OTP_DISTURBED1 is used to report disturbed state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_DISTURBED2 is used to report disturbed state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_disturbed0](bsec_otp_disturbed0) module"]
pub type BSEC_OTP_DISTURBED0 = crate::Reg<u32, _BSEC_OTP_DISTURBED0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DISTURBED0;
#[doc = "`read()` method returns [bsec_otp_disturbed0::R](bsec_otp_disturbed0::R) reader structure"]
impl crate::Readable for BSEC_OTP_DISTURBED0 {}
#[doc = "BSEC_OTP_DISTURBED0 is used to report disturbed state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP). BSEC_OTP_DISTURBED1 is used to report disturbed state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_DISTURBED2 is used to report disturbed state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95."]
pub mod bsec_otp_disturbed0;
#[doc = "BSEC_OTP_DISTURBED0 is used to report disturbed state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP). BSEC_OTP_DISTURBED1 is used to report disturbed state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_DISTURBED2 is used to report disturbed state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_disturbed1](bsec_otp_disturbed1) module"]
pub type BSEC_OTP_DISTURBED1 = crate::Reg<u32, _BSEC_OTP_DISTURBED1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DISTURBED1;
#[doc = "`read()` method returns [bsec_otp_disturbed1::R](bsec_otp_disturbed1::R) reader structure"]
impl crate::Readable for BSEC_OTP_DISTURBED1 {}
#[doc = "BSEC_OTP_DISTURBED0 is used to report disturbed state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP). BSEC_OTP_DISTURBED1 is used to report disturbed state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_DISTURBED2 is used to report disturbed state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95."]
pub mod bsec_otp_disturbed1;
#[doc = "BSEC_OTP_DISTURBED0 is used to report disturbed state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP). BSEC_OTP_DISTURBED1 is used to report disturbed state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_DISTURBED2 is used to report disturbed state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_disturbed2](bsec_otp_disturbed2) module"]
pub type BSEC_OTP_DISTURBED2 = crate::Reg<u32, _BSEC_OTP_DISTURBED2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DISTURBED2;
#[doc = "`read()` method returns [bsec_otp_disturbed2::R](bsec_otp_disturbed2::R) reader structure"]
impl crate::Readable for BSEC_OTP_DISTURBED2 {}
#[doc = "BSEC_OTP_DISTURBED0 is used to report disturbed state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP). BSEC_OTP_DISTURBED1 is used to report disturbed state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_DISTURBED2 is used to report disturbed state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95."]
pub mod bsec_otp_disturbed2;
#[doc = "BSEC_OTP_ERROR0 is used to report error state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP which are protected by 2:1 redundancy). BSEC_OTP_ERROR1 is used to report error state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 which are protected by 6-bit ECC. BSEC_OTP_ERROR2 is used to report error state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 which are protected by 6-bit ECC.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_error0](bsec_otp_error0) module"]
pub type BSEC_OTP_ERROR0 = crate::Reg<u32, _BSEC_OTP_ERROR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_ERROR0;
#[doc = "`read()` method returns [bsec_otp_error0::R](bsec_otp_error0::R) reader structure"]
impl crate::Readable for BSEC_OTP_ERROR0 {}
#[doc = "BSEC_OTP_ERROR0 is used to report error state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP which are protected by 2:1 redundancy). BSEC_OTP_ERROR1 is used to report error state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 which are protected by 6-bit ECC. BSEC_OTP_ERROR2 is used to report error state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 which are protected by 6-bit ECC."]
pub mod bsec_otp_error0;
#[doc = "BSEC_OTP_ERROR0 is used to report error state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP which are protected by 2:1 redundancy). BSEC_OTP_ERROR1 is used to report error state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 which are protected by 6-bit ECC. BSEC_OTP_ERROR2 is used to report error state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 which are protected by 6-bit ECC.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_error1](bsec_otp_error1) module"]
pub type BSEC_OTP_ERROR1 = crate::Reg<u32, _BSEC_OTP_ERROR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_ERROR1;
#[doc = "`read()` method returns [bsec_otp_error1::R](bsec_otp_error1::R) reader structure"]
impl crate::Readable for BSEC_OTP_ERROR1 {}
#[doc = "BSEC_OTP_ERROR0 is used to report error state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP which are protected by 2:1 redundancy). BSEC_OTP_ERROR1 is used to report error state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 which are protected by 6-bit ECC. BSEC_OTP_ERROR2 is used to report error state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 which are protected by 6-bit ECC."]
pub mod bsec_otp_error1;
#[doc = "BSEC_OTP_ERROR0 is used to report error state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP which are protected by 2:1 redundancy). BSEC_OTP_ERROR1 is used to report error state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 which are protected by 6-bit ECC. BSEC_OTP_ERROR2 is used to report error state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 which are protected by 6-bit ECC.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_error2](bsec_otp_error2) module"]
pub type BSEC_OTP_ERROR2 = crate::Reg<u32, _BSEC_OTP_ERROR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_ERROR2;
#[doc = "`read()` method returns [bsec_otp_error2::R](bsec_otp_error2::R) reader structure"]
impl crate::Readable for BSEC_OTP_ERROR2 {}
#[doc = "BSEC_OTP_ERROR0 is used to report error state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP which are protected by 2:1 redundancy). BSEC_OTP_ERROR1 is used to report error state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 which are protected by 6-bit ECC. BSEC_OTP_ERROR2 is used to report error state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 which are protected by 6-bit ECC."]
pub mod bsec_otp_error2;
#[doc = "BSEC_OTP_WLOCK0 is used to report permanent write lock of BSEC_OTP_DATA0 to BSEC_OTP_DATA31. BSEC_OTP_WLOCK1 is used to report permanent write lock of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_WLOCK2 is used to report permanent write lock of BSEC_OTP_DATA64 to BSEC_OTP_DATA95. Permanent write lock requires a programming sequence to lock a word (see section:Section3.3.6: OTP operations on page178).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_wrlock0](bsec_otp_wrlock0) module"]
pub type BSEC_OTP_WRLOCK0 = crate::Reg<u32, _BSEC_OTP_WRLOCK0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_WRLOCK0;
#[doc = "`read()` method returns [bsec_otp_wrlock0::R](bsec_otp_wrlock0::R) reader structure"]
impl crate::Readable for BSEC_OTP_WRLOCK0 {}
#[doc = "BSEC_OTP_WLOCK0 is used to report permanent write lock of BSEC_OTP_DATA0 to BSEC_OTP_DATA31. BSEC_OTP_WLOCK1 is used to report permanent write lock of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_WLOCK2 is used to report permanent write lock of BSEC_OTP_DATA64 to BSEC_OTP_DATA95. Permanent write lock requires a programming sequence to lock a word (see section:Section3.3.6: OTP operations on page178)."]
pub mod bsec_otp_wrlock0;
#[doc = "BSEC_OTP_WLOCK0 is used to report permanent write lock of BSEC_OTP_DATA0 to BSEC_OTP_DATA31. BSEC_OTP_WLOCK1 is used to report permanent write lock of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_WLOCK2 is used to report permanent write lock of BSEC_OTP_DATA64 to BSEC_OTP_DATA95. Permanent write lock requires a programming sequence to lock a word (see section:Section3.3.6: OTP operations on page178).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_wrlock1](bsec_otp_wrlock1) module"]
pub type BSEC_OTP_WRLOCK1 = crate::Reg<u32, _BSEC_OTP_WRLOCK1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_WRLOCK1;
#[doc = "`read()` method returns [bsec_otp_wrlock1::R](bsec_otp_wrlock1::R) reader structure"]
impl crate::Readable for BSEC_OTP_WRLOCK1 {}
#[doc = "BSEC_OTP_WLOCK0 is used to report permanent write lock of BSEC_OTP_DATA0 to BSEC_OTP_DATA31. BSEC_OTP_WLOCK1 is used to report permanent write lock of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_WLOCK2 is used to report permanent write lock of BSEC_OTP_DATA64 to BSEC_OTP_DATA95. Permanent write lock requires a programming sequence to lock a word (see section:Section3.3.6: OTP operations on page178)."]
pub mod bsec_otp_wrlock1;
#[doc = "BSEC_OTP_WLOCK0 is used to report permanent write lock of BSEC_OTP_DATA0 to BSEC_OTP_DATA31. BSEC_OTP_WLOCK1 is used to report permanent write lock of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_WLOCK2 is used to report permanent write lock of BSEC_OTP_DATA64 to BSEC_OTP_DATA95. Permanent write lock requires a programming sequence to lock a word (see section:Section3.3.6: OTP operations on page178).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_wrlock2](bsec_otp_wrlock2) module"]
pub type BSEC_OTP_WRLOCK2 = crate::Reg<u32, _BSEC_OTP_WRLOCK2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_WRLOCK2;
#[doc = "`read()` method returns [bsec_otp_wrlock2::R](bsec_otp_wrlock2::R) reader structure"]
impl crate::Readable for BSEC_OTP_WRLOCK2 {}
#[doc = "BSEC_OTP_WLOCK0 is used to report permanent write lock of BSEC_OTP_DATA0 to BSEC_OTP_DATA31. BSEC_OTP_WLOCK1 is used to report permanent write lock of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_WLOCK2 is used to report permanent write lock of BSEC_OTP_DATA64 to BSEC_OTP_DATA95. Permanent write lock requires a programming sequence to lock a word (see section:Section3.3.6: OTP operations on page178)."]
pub mod bsec_otp_wrlock2;
#[doc = "BSEC_OTP_SPLOCK0 is used to lock the programming of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset BSEC_OTP_SPLOCK1 is used to lock the programming of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset BSEC_OTP_SPLOCK2 is used to lock the programming of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset Attempt to sticky program locked OTP word are silently ignored.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_splock0](bsec_otp_splock0) module"]
pub type BSEC_OTP_SPLOCK0 = crate::Reg<u32, _BSEC_OTP_SPLOCK0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_SPLOCK0;
#[doc = "`read()` method returns [bsec_otp_splock0::R](bsec_otp_splock0::R) reader structure"]
impl crate::Readable for BSEC_OTP_SPLOCK0 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_splock0::W](bsec_otp_splock0::W) writer structure"]
impl crate::Writable for BSEC_OTP_SPLOCK0 {}
#[doc = "BSEC_OTP_SPLOCK0 is used to lock the programming of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset BSEC_OTP_SPLOCK1 is used to lock the programming of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset BSEC_OTP_SPLOCK2 is used to lock the programming of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset Attempt to sticky program locked OTP word are silently ignored."]
pub mod bsec_otp_splock0;
#[doc = "BSEC_OTP_SPLOCK0 is used to lock the programming of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset BSEC_OTP_SPLOCK1 is used to lock the programming of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset BSEC_OTP_SPLOCK2 is used to lock the programming of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset Attempt to sticky program locked OTP word are silently ignored.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_splock1](bsec_otp_splock1) module"]
pub type BSEC_OTP_SPLOCK1 = crate::Reg<u32, _BSEC_OTP_SPLOCK1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_SPLOCK1;
#[doc = "`read()` method returns [bsec_otp_splock1::R](bsec_otp_splock1::R) reader structure"]
impl crate::Readable for BSEC_OTP_SPLOCK1 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_splock1::W](bsec_otp_splock1::W) writer structure"]
impl crate::Writable for BSEC_OTP_SPLOCK1 {}
#[doc = "BSEC_OTP_SPLOCK0 is used to lock the programming of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset BSEC_OTP_SPLOCK1 is used to lock the programming of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset BSEC_OTP_SPLOCK2 is used to lock the programming of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset Attempt to sticky program locked OTP word are silently ignored."]
pub mod bsec_otp_splock1;
#[doc = "BSEC_OTP_SPLOCK0 is used to lock the programming of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset BSEC_OTP_SPLOCK1 is used to lock the programming of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset BSEC_OTP_SPLOCK2 is used to lock the programming of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset Attempt to sticky program locked OTP word are silently ignored.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_splock2](bsec_otp_splock2) module"]
pub type BSEC_OTP_SPLOCK2 = crate::Reg<u32, _BSEC_OTP_SPLOCK2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_SPLOCK2;
#[doc = "`read()` method returns [bsec_otp_splock2::R](bsec_otp_splock2::R) reader structure"]
impl crate::Readable for BSEC_OTP_SPLOCK2 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_splock2::W](bsec_otp_splock2::W) writer structure"]
impl crate::Writable for BSEC_OTP_SPLOCK2 {}
#[doc = "BSEC_OTP_SPLOCK0 is used to lock the programming of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset BSEC_OTP_SPLOCK1 is used to lock the programming of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset BSEC_OTP_SPLOCK2 is used to lock the programming of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset Attempt to sticky program locked OTP word are silently ignored."]
pub mod bsec_otp_splock2;
#[doc = "BSEC_OTP_SWLOCK0 is used to prevent writing to BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SWLOCK1 is used to prevent writing to BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SWLOCK2 is used to prevent writing to BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Write to shadow write locked BSEC_OTP_DATA word are silently ignored. Writing to OTP word 0 shadow is always prevented.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_swlock0](bsec_otp_swlock0) module"]
pub type BSEC_OTP_SWLOCK0 = crate::Reg<u32, _BSEC_OTP_SWLOCK0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_SWLOCK0;
#[doc = "`read()` method returns [bsec_otp_swlock0::R](bsec_otp_swlock0::R) reader structure"]
impl crate::Readable for BSEC_OTP_SWLOCK0 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_swlock0::W](bsec_otp_swlock0::W) writer structure"]
impl crate::Writable for BSEC_OTP_SWLOCK0 {}
#[doc = "BSEC_OTP_SWLOCK0 is used to prevent writing to BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SWLOCK1 is used to prevent writing to BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SWLOCK2 is used to prevent writing to BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Write to shadow write locked BSEC_OTP_DATA word are silently ignored. Writing to OTP word 0 shadow is always prevented."]
pub mod bsec_otp_swlock0;
#[doc = "BSEC_OTP_SWLOCK0 is used to prevent writing to BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SWLOCK1 is used to prevent writing to BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SWLOCK2 is used to prevent writing to BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Write to shadow write locked BSEC_OTP_DATA word are silently ignored. Writing to OTP word 0 shadow is always prevented.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_swlock1](bsec_otp_swlock1) module"]
pub type BSEC_OTP_SWLOCK1 = crate::Reg<u32, _BSEC_OTP_SWLOCK1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_SWLOCK1;
#[doc = "`read()` method returns [bsec_otp_swlock1::R](bsec_otp_swlock1::R) reader structure"]
impl crate::Readable for BSEC_OTP_SWLOCK1 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_swlock1::W](bsec_otp_swlock1::W) writer structure"]
impl crate::Writable for BSEC_OTP_SWLOCK1 {}
#[doc = "BSEC_OTP_SWLOCK0 is used to prevent writing to BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SWLOCK1 is used to prevent writing to BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SWLOCK2 is used to prevent writing to BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Write to shadow write locked BSEC_OTP_DATA word are silently ignored. Writing to OTP word 0 shadow is always prevented."]
pub mod bsec_otp_swlock1;
#[doc = "BSEC_OTP_SWLOCK0 is used to prevent writing to BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SWLOCK1 is used to prevent writing to BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SWLOCK2 is used to prevent writing to BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Write to shadow write locked BSEC_OTP_DATA word are silently ignored. Writing to OTP word 0 shadow is always prevented.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_swlock2](bsec_otp_swlock2) module"]
pub type BSEC_OTP_SWLOCK2 = crate::Reg<u32, _BSEC_OTP_SWLOCK2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_SWLOCK2;
#[doc = "`read()` method returns [bsec_otp_swlock2::R](bsec_otp_swlock2::R) reader structure"]
impl crate::Readable for BSEC_OTP_SWLOCK2 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_swlock2::W](bsec_otp_swlock2::W) writer structure"]
impl crate::Writable for BSEC_OTP_SWLOCK2 {}
#[doc = "BSEC_OTP_SWLOCK0 is used to prevent writing to BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SWLOCK1 is used to prevent writing to BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SWLOCK2 is used to prevent writing to BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Write to shadow write locked BSEC_OTP_DATA word are silently ignored. Writing to OTP word 0 shadow is always prevented."]
pub mod bsec_otp_swlock2;
#[doc = "BSEC_OTP_SRLOCK0 is used to prevent reloading of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SRLOCK1 is used to prevent reloading of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SRLOCK2 is used to prevent reloading of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Setting SRLOCK bits or attempt to reload a locked OTP do not clear the corresponding BSEC_OTP_DATAx shadow register. BSEC_OTP_SRLOCK0 bit 0 is controlled by hardware according to fuse_ok, writing to this bit has no effect.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_srlock0](bsec_otp_srlock0) module"]
pub type BSEC_OTP_SRLOCK0 = crate::Reg<u32, _BSEC_OTP_SRLOCK0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_SRLOCK0;
#[doc = "`read()` method returns [bsec_otp_srlock0::R](bsec_otp_srlock0::R) reader structure"]
impl crate::Readable for BSEC_OTP_SRLOCK0 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_srlock0::W](bsec_otp_srlock0::W) writer structure"]
impl crate::Writable for BSEC_OTP_SRLOCK0 {}
#[doc = "BSEC_OTP_SRLOCK0 is used to prevent reloading of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SRLOCK1 is used to prevent reloading of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SRLOCK2 is used to prevent reloading of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Setting SRLOCK bits or attempt to reload a locked OTP do not clear the corresponding BSEC_OTP_DATAx shadow register. BSEC_OTP_SRLOCK0 bit 0 is controlled by hardware according to fuse_ok, writing to this bit has no effect."]
pub mod bsec_otp_srlock0;
#[doc = "BSEC_OTP_SRLOCK0 is used to prevent reloading of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SRLOCK1 is used to prevent reloading of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SRLOCK2 is used to prevent reloading of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Setting SRLOCK bits or attempt to reload a locked OTP do not clear the corresponding BSEC_OTP_DATAx shadow register. BSEC_OTP_SRLOCK0 bit 0 is controlled by hardware according to fuse_ok, writing to this bit has no effect.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_srlock1](bsec_otp_srlock1) module"]
pub type BSEC_OTP_SRLOCK1 = crate::Reg<u32, _BSEC_OTP_SRLOCK1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_SRLOCK1;
#[doc = "`read()` method returns [bsec_otp_srlock1::R](bsec_otp_srlock1::R) reader structure"]
impl crate::Readable for BSEC_OTP_SRLOCK1 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_srlock1::W](bsec_otp_srlock1::W) writer structure"]
impl crate::Writable for BSEC_OTP_SRLOCK1 {}
#[doc = "BSEC_OTP_SRLOCK0 is used to prevent reloading of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SRLOCK1 is used to prevent reloading of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SRLOCK2 is used to prevent reloading of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Setting SRLOCK bits or attempt to reload a locked OTP do not clear the corresponding BSEC_OTP_DATAx shadow register. BSEC_OTP_SRLOCK0 bit 0 is controlled by hardware according to fuse_ok, writing to this bit has no effect."]
pub mod bsec_otp_srlock1;
#[doc = "BSEC_OTP_SRLOCK0 is used to prevent reloading of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SRLOCK1 is used to prevent reloading of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SRLOCK2 is used to prevent reloading of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Setting SRLOCK bits or attempt to reload a locked OTP do not clear the corresponding BSEC_OTP_DATAx shadow register. BSEC_OTP_SRLOCK0 bit 0 is controlled by hardware according to fuse_ok, writing to this bit has no effect.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_srlock2](bsec_otp_srlock2) module"]
pub type BSEC_OTP_SRLOCK2 = crate::Reg<u32, _BSEC_OTP_SRLOCK2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_SRLOCK2;
#[doc = "`read()` method returns [bsec_otp_srlock2::R](bsec_otp_srlock2::R) reader structure"]
impl crate::Readable for BSEC_OTP_SRLOCK2 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_srlock2::W](bsec_otp_srlock2::W) writer structure"]
impl crate::Writable for BSEC_OTP_SRLOCK2 {}
#[doc = "BSEC_OTP_SRLOCK0 is used to prevent reloading of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SRLOCK1 is used to prevent reloading of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SRLOCK2 is used to prevent reloading of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Setting SRLOCK bits or attempt to reload a locked OTP do not clear the corresponding BSEC_OTP_DATAx shadow register. BSEC_OTP_SRLOCK0 bit 0 is controlled by hardware according to fuse_ok, writing to this bit has no effect."]
pub mod bsec_otp_srlock2;
#[doc = "BSEC JTAG input register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_jtagin](bsec_jtagin) module"]
pub type BSEC_JTAGIN = crate::Reg<u32, _BSEC_JTAGIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_JTAGIN;
#[doc = "`read()` method returns [bsec_jtagin::R](bsec_jtagin::R) reader structure"]
impl crate::Readable for BSEC_JTAGIN {}
#[doc = "BSEC JTAG input register"]
pub mod bsec_jtagin;
#[doc = "BSEC JTAG output register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_jtagout](bsec_jtagout) module"]
pub type BSEC_JTAGOUT = crate::Reg<u32, _BSEC_JTAGOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_JTAGOUT;
#[doc = "`read()` method returns [bsec_jtagout::R](bsec_jtagout::R) reader structure"]
impl crate::Readable for BSEC_JTAGOUT {}
#[doc = "`write(|w| ..)` method takes [bsec_jtagout::W](bsec_jtagout::W) writer structure"]
impl crate::Writable for BSEC_JTAGOUT {}
#[doc = "BSEC JTAG output register"]
pub mod bsec_jtagout;
#[doc = "BSEC scratch register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_scratch](bsec_scratch) module"]
pub type BSEC_SCRATCH = crate::Reg<u32, _BSEC_SCRATCH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_SCRATCH;
#[doc = "`read()` method returns [bsec_scratch::R](bsec_scratch::R) reader structure"]
impl crate::Readable for BSEC_SCRATCH {}
#[doc = "`write(|w| ..)` method takes [bsec_scratch::W](bsec_scratch::W) writer structure"]
impl crate::Writable for BSEC_SCRATCH {}
#[doc = "BSEC scratch register"]
pub mod bsec_scratch;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data0](bsec_otp_data0) module"]
pub type BSEC_OTP_DATA0 = crate::Reg<u32, _BSEC_OTP_DATA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA0;
#[doc = "`read()` method returns [bsec_otp_data0::R](bsec_otp_data0::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA0 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data0::W](bsec_otp_data0::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA0 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data0;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data1](bsec_otp_data1) module"]
pub type BSEC_OTP_DATA1 = crate::Reg<u32, _BSEC_OTP_DATA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA1;
#[doc = "`read()` method returns [bsec_otp_data1::R](bsec_otp_data1::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA1 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data1::W](bsec_otp_data1::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA1 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data1;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data2](bsec_otp_data2) module"]
pub type BSEC_OTP_DATA2 = crate::Reg<u32, _BSEC_OTP_DATA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA2;
#[doc = "`read()` method returns [bsec_otp_data2::R](bsec_otp_data2::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA2 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data2::W](bsec_otp_data2::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA2 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data2;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data3](bsec_otp_data3) module"]
pub type BSEC_OTP_DATA3 = crate::Reg<u32, _BSEC_OTP_DATA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA3;
#[doc = "`read()` method returns [bsec_otp_data3::R](bsec_otp_data3::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA3 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data3::W](bsec_otp_data3::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA3 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data3;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data4](bsec_otp_data4) module"]
pub type BSEC_OTP_DATA4 = crate::Reg<u32, _BSEC_OTP_DATA4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA4;
#[doc = "`read()` method returns [bsec_otp_data4::R](bsec_otp_data4::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA4 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data4::W](bsec_otp_data4::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA4 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data4;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data5](bsec_otp_data5) module"]
pub type BSEC_OTP_DATA5 = crate::Reg<u32, _BSEC_OTP_DATA5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA5;
#[doc = "`read()` method returns [bsec_otp_data5::R](bsec_otp_data5::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA5 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data5::W](bsec_otp_data5::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA5 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data5;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data6](bsec_otp_data6) module"]
pub type BSEC_OTP_DATA6 = crate::Reg<u32, _BSEC_OTP_DATA6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA6;
#[doc = "`read()` method returns [bsec_otp_data6::R](bsec_otp_data6::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA6 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data6::W](bsec_otp_data6::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA6 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data6;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data7](bsec_otp_data7) module"]
pub type BSEC_OTP_DATA7 = crate::Reg<u32, _BSEC_OTP_DATA7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA7;
#[doc = "`read()` method returns [bsec_otp_data7::R](bsec_otp_data7::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA7 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data7::W](bsec_otp_data7::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA7 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data7;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data8](bsec_otp_data8) module"]
pub type BSEC_OTP_DATA8 = crate::Reg<u32, _BSEC_OTP_DATA8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA8;
#[doc = "`read()` method returns [bsec_otp_data8::R](bsec_otp_data8::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA8 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data8::W](bsec_otp_data8::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA8 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data8;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data9](bsec_otp_data9) module"]
pub type BSEC_OTP_DATA9 = crate::Reg<u32, _BSEC_OTP_DATA9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA9;
#[doc = "`read()` method returns [bsec_otp_data9::R](bsec_otp_data9::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA9 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data9::W](bsec_otp_data9::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA9 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data9;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data10](bsec_otp_data10) module"]
pub type BSEC_OTP_DATA10 = crate::Reg<u32, _BSEC_OTP_DATA10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA10;
#[doc = "`read()` method returns [bsec_otp_data10::R](bsec_otp_data10::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA10 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data10::W](bsec_otp_data10::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA10 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data10;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data11](bsec_otp_data11) module"]
pub type BSEC_OTP_DATA11 = crate::Reg<u32, _BSEC_OTP_DATA11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA11;
#[doc = "`read()` method returns [bsec_otp_data11::R](bsec_otp_data11::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA11 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data11::W](bsec_otp_data11::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA11 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data11;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data12](bsec_otp_data12) module"]
pub type BSEC_OTP_DATA12 = crate::Reg<u32, _BSEC_OTP_DATA12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA12;
#[doc = "`read()` method returns [bsec_otp_data12::R](bsec_otp_data12::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA12 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data12::W](bsec_otp_data12::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA12 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data12;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data13](bsec_otp_data13) module"]
pub type BSEC_OTP_DATA13 = crate::Reg<u32, _BSEC_OTP_DATA13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA13;
#[doc = "`read()` method returns [bsec_otp_data13::R](bsec_otp_data13::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA13 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data13::W](bsec_otp_data13::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA13 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data13;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data14](bsec_otp_data14) module"]
pub type BSEC_OTP_DATA14 = crate::Reg<u32, _BSEC_OTP_DATA14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA14;
#[doc = "`read()` method returns [bsec_otp_data14::R](bsec_otp_data14::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA14 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data14::W](bsec_otp_data14::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA14 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data14;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data15](bsec_otp_data15) module"]
pub type BSEC_OTP_DATA15 = crate::Reg<u32, _BSEC_OTP_DATA15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA15;
#[doc = "`read()` method returns [bsec_otp_data15::R](bsec_otp_data15::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA15 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data15::W](bsec_otp_data15::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA15 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data15;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data16](bsec_otp_data16) module"]
pub type BSEC_OTP_DATA16 = crate::Reg<u32, _BSEC_OTP_DATA16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA16;
#[doc = "`read()` method returns [bsec_otp_data16::R](bsec_otp_data16::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA16 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data16::W](bsec_otp_data16::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA16 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data16;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data17](bsec_otp_data17) module"]
pub type BSEC_OTP_DATA17 = crate::Reg<u32, _BSEC_OTP_DATA17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA17;
#[doc = "`read()` method returns [bsec_otp_data17::R](bsec_otp_data17::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA17 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data17::W](bsec_otp_data17::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA17 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data17;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data18](bsec_otp_data18) module"]
pub type BSEC_OTP_DATA18 = crate::Reg<u32, _BSEC_OTP_DATA18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA18;
#[doc = "`read()` method returns [bsec_otp_data18::R](bsec_otp_data18::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA18 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data18::W](bsec_otp_data18::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA18 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data18;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data19](bsec_otp_data19) module"]
pub type BSEC_OTP_DATA19 = crate::Reg<u32, _BSEC_OTP_DATA19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA19;
#[doc = "`read()` method returns [bsec_otp_data19::R](bsec_otp_data19::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA19 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data19::W](bsec_otp_data19::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA19 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data19;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data20](bsec_otp_data20) module"]
pub type BSEC_OTP_DATA20 = crate::Reg<u32, _BSEC_OTP_DATA20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA20;
#[doc = "`read()` method returns [bsec_otp_data20::R](bsec_otp_data20::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA20 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data20::W](bsec_otp_data20::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA20 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data20;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data21](bsec_otp_data21) module"]
pub type BSEC_OTP_DATA21 = crate::Reg<u32, _BSEC_OTP_DATA21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA21;
#[doc = "`read()` method returns [bsec_otp_data21::R](bsec_otp_data21::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA21 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data21::W](bsec_otp_data21::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA21 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data21;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data22](bsec_otp_data22) module"]
pub type BSEC_OTP_DATA22 = crate::Reg<u32, _BSEC_OTP_DATA22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA22;
#[doc = "`read()` method returns [bsec_otp_data22::R](bsec_otp_data22::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA22 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data22::W](bsec_otp_data22::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA22 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data22;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data23](bsec_otp_data23) module"]
pub type BSEC_OTP_DATA23 = crate::Reg<u32, _BSEC_OTP_DATA23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA23;
#[doc = "`read()` method returns [bsec_otp_data23::R](bsec_otp_data23::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA23 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data23::W](bsec_otp_data23::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA23 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data23;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data24](bsec_otp_data24) module"]
pub type BSEC_OTP_DATA24 = crate::Reg<u32, _BSEC_OTP_DATA24>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA24;
#[doc = "`read()` method returns [bsec_otp_data24::R](bsec_otp_data24::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA24 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data24::W](bsec_otp_data24::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA24 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data24;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data25](bsec_otp_data25) module"]
pub type BSEC_OTP_DATA25 = crate::Reg<u32, _BSEC_OTP_DATA25>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA25;
#[doc = "`read()` method returns [bsec_otp_data25::R](bsec_otp_data25::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA25 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data25::W](bsec_otp_data25::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA25 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data25;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data26](bsec_otp_data26) module"]
pub type BSEC_OTP_DATA26 = crate::Reg<u32, _BSEC_OTP_DATA26>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA26;
#[doc = "`read()` method returns [bsec_otp_data26::R](bsec_otp_data26::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA26 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data26::W](bsec_otp_data26::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA26 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data26;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data27](bsec_otp_data27) module"]
pub type BSEC_OTP_DATA27 = crate::Reg<u32, _BSEC_OTP_DATA27>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA27;
#[doc = "`read()` method returns [bsec_otp_data27::R](bsec_otp_data27::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA27 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data27::W](bsec_otp_data27::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA27 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data27;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data28](bsec_otp_data28) module"]
pub type BSEC_OTP_DATA28 = crate::Reg<u32, _BSEC_OTP_DATA28>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA28;
#[doc = "`read()` method returns [bsec_otp_data28::R](bsec_otp_data28::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA28 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data28::W](bsec_otp_data28::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA28 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data28;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data29](bsec_otp_data29) module"]
pub type BSEC_OTP_DATA29 = crate::Reg<u32, _BSEC_OTP_DATA29>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA29;
#[doc = "`read()` method returns [bsec_otp_data29::R](bsec_otp_data29::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA29 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data29::W](bsec_otp_data29::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA29 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data29;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data30](bsec_otp_data30) module"]
pub type BSEC_OTP_DATA30 = crate::Reg<u32, _BSEC_OTP_DATA30>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA30;
#[doc = "`read()` method returns [bsec_otp_data30::R](bsec_otp_data30::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA30 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data30::W](bsec_otp_data30::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA30 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data30;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data31](bsec_otp_data31) module"]
pub type BSEC_OTP_DATA31 = crate::Reg<u32, _BSEC_OTP_DATA31>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA31;
#[doc = "`read()` method returns [bsec_otp_data31::R](bsec_otp_data31::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA31 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data31::W](bsec_otp_data31::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA31 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data31;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data32](bsec_otp_data32) module"]
pub type BSEC_OTP_DATA32 = crate::Reg<u32, _BSEC_OTP_DATA32>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA32;
#[doc = "`read()` method returns [bsec_otp_data32::R](bsec_otp_data32::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA32 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data32::W](bsec_otp_data32::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA32 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data32;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data33](bsec_otp_data33) module"]
pub type BSEC_OTP_DATA33 = crate::Reg<u32, _BSEC_OTP_DATA33>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA33;
#[doc = "`read()` method returns [bsec_otp_data33::R](bsec_otp_data33::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA33 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data33::W](bsec_otp_data33::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA33 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data33;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data34](bsec_otp_data34) module"]
pub type BSEC_OTP_DATA34 = crate::Reg<u32, _BSEC_OTP_DATA34>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA34;
#[doc = "`read()` method returns [bsec_otp_data34::R](bsec_otp_data34::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA34 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data34::W](bsec_otp_data34::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA34 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data34;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data35](bsec_otp_data35) module"]
pub type BSEC_OTP_DATA35 = crate::Reg<u32, _BSEC_OTP_DATA35>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA35;
#[doc = "`read()` method returns [bsec_otp_data35::R](bsec_otp_data35::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA35 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data35::W](bsec_otp_data35::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA35 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data35;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data36](bsec_otp_data36) module"]
pub type BSEC_OTP_DATA36 = crate::Reg<u32, _BSEC_OTP_DATA36>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA36;
#[doc = "`read()` method returns [bsec_otp_data36::R](bsec_otp_data36::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA36 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data36::W](bsec_otp_data36::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA36 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data36;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data37](bsec_otp_data37) module"]
pub type BSEC_OTP_DATA37 = crate::Reg<u32, _BSEC_OTP_DATA37>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA37;
#[doc = "`read()` method returns [bsec_otp_data37::R](bsec_otp_data37::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA37 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data37::W](bsec_otp_data37::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA37 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data37;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data38](bsec_otp_data38) module"]
pub type BSEC_OTP_DATA38 = crate::Reg<u32, _BSEC_OTP_DATA38>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA38;
#[doc = "`read()` method returns [bsec_otp_data38::R](bsec_otp_data38::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA38 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data38::W](bsec_otp_data38::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA38 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data38;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data39](bsec_otp_data39) module"]
pub type BSEC_OTP_DATA39 = crate::Reg<u32, _BSEC_OTP_DATA39>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA39;
#[doc = "`read()` method returns [bsec_otp_data39::R](bsec_otp_data39::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA39 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data39::W](bsec_otp_data39::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA39 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data39;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data40](bsec_otp_data40) module"]
pub type BSEC_OTP_DATA40 = crate::Reg<u32, _BSEC_OTP_DATA40>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA40;
#[doc = "`read()` method returns [bsec_otp_data40::R](bsec_otp_data40::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA40 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data40::W](bsec_otp_data40::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA40 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data40;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data41](bsec_otp_data41) module"]
pub type BSEC_OTP_DATA41 = crate::Reg<u32, _BSEC_OTP_DATA41>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA41;
#[doc = "`read()` method returns [bsec_otp_data41::R](bsec_otp_data41::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA41 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data41::W](bsec_otp_data41::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA41 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data41;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data42](bsec_otp_data42) module"]
pub type BSEC_OTP_DATA42 = crate::Reg<u32, _BSEC_OTP_DATA42>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA42;
#[doc = "`read()` method returns [bsec_otp_data42::R](bsec_otp_data42::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA42 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data42::W](bsec_otp_data42::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA42 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data42;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data43](bsec_otp_data43) module"]
pub type BSEC_OTP_DATA43 = crate::Reg<u32, _BSEC_OTP_DATA43>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA43;
#[doc = "`read()` method returns [bsec_otp_data43::R](bsec_otp_data43::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA43 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data43::W](bsec_otp_data43::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA43 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data43;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data44](bsec_otp_data44) module"]
pub type BSEC_OTP_DATA44 = crate::Reg<u32, _BSEC_OTP_DATA44>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA44;
#[doc = "`read()` method returns [bsec_otp_data44::R](bsec_otp_data44::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA44 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data44::W](bsec_otp_data44::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA44 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data44;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data45](bsec_otp_data45) module"]
pub type BSEC_OTP_DATA45 = crate::Reg<u32, _BSEC_OTP_DATA45>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA45;
#[doc = "`read()` method returns [bsec_otp_data45::R](bsec_otp_data45::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA45 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data45::W](bsec_otp_data45::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA45 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data45;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data46](bsec_otp_data46) module"]
pub type BSEC_OTP_DATA46 = crate::Reg<u32, _BSEC_OTP_DATA46>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA46;
#[doc = "`read()` method returns [bsec_otp_data46::R](bsec_otp_data46::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA46 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data46::W](bsec_otp_data46::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA46 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data46;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data47](bsec_otp_data47) module"]
pub type BSEC_OTP_DATA47 = crate::Reg<u32, _BSEC_OTP_DATA47>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA47;
#[doc = "`read()` method returns [bsec_otp_data47::R](bsec_otp_data47::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA47 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data47::W](bsec_otp_data47::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA47 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data47;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data48](bsec_otp_data48) module"]
pub type BSEC_OTP_DATA48 = crate::Reg<u32, _BSEC_OTP_DATA48>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA48;
#[doc = "`read()` method returns [bsec_otp_data48::R](bsec_otp_data48::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA48 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data48::W](bsec_otp_data48::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA48 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data48;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data49](bsec_otp_data49) module"]
pub type BSEC_OTP_DATA49 = crate::Reg<u32, _BSEC_OTP_DATA49>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA49;
#[doc = "`read()` method returns [bsec_otp_data49::R](bsec_otp_data49::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA49 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data49::W](bsec_otp_data49::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA49 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data49;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data50](bsec_otp_data50) module"]
pub type BSEC_OTP_DATA50 = crate::Reg<u32, _BSEC_OTP_DATA50>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA50;
#[doc = "`read()` method returns [bsec_otp_data50::R](bsec_otp_data50::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA50 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data50::W](bsec_otp_data50::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA50 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data50;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data51](bsec_otp_data51) module"]
pub type BSEC_OTP_DATA51 = crate::Reg<u32, _BSEC_OTP_DATA51>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA51;
#[doc = "`read()` method returns [bsec_otp_data51::R](bsec_otp_data51::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA51 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data51::W](bsec_otp_data51::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA51 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data51;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data52](bsec_otp_data52) module"]
pub type BSEC_OTP_DATA52 = crate::Reg<u32, _BSEC_OTP_DATA52>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA52;
#[doc = "`read()` method returns [bsec_otp_data52::R](bsec_otp_data52::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA52 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data52::W](bsec_otp_data52::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA52 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data52;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data53](bsec_otp_data53) module"]
pub type BSEC_OTP_DATA53 = crate::Reg<u32, _BSEC_OTP_DATA53>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA53;
#[doc = "`read()` method returns [bsec_otp_data53::R](bsec_otp_data53::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA53 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data53::W](bsec_otp_data53::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA53 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data53;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data54](bsec_otp_data54) module"]
pub type BSEC_OTP_DATA54 = crate::Reg<u32, _BSEC_OTP_DATA54>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA54;
#[doc = "`read()` method returns [bsec_otp_data54::R](bsec_otp_data54::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA54 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data54::W](bsec_otp_data54::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA54 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data54;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data55](bsec_otp_data55) module"]
pub type BSEC_OTP_DATA55 = crate::Reg<u32, _BSEC_OTP_DATA55>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA55;
#[doc = "`read()` method returns [bsec_otp_data55::R](bsec_otp_data55::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA55 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data55::W](bsec_otp_data55::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA55 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data55;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data56](bsec_otp_data56) module"]
pub type BSEC_OTP_DATA56 = crate::Reg<u32, _BSEC_OTP_DATA56>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA56;
#[doc = "`read()` method returns [bsec_otp_data56::R](bsec_otp_data56::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA56 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data56::W](bsec_otp_data56::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA56 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data56;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data57](bsec_otp_data57) module"]
pub type BSEC_OTP_DATA57 = crate::Reg<u32, _BSEC_OTP_DATA57>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA57;
#[doc = "`read()` method returns [bsec_otp_data57::R](bsec_otp_data57::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA57 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data57::W](bsec_otp_data57::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA57 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data57;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data58](bsec_otp_data58) module"]
pub type BSEC_OTP_DATA58 = crate::Reg<u32, _BSEC_OTP_DATA58>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA58;
#[doc = "`read()` method returns [bsec_otp_data58::R](bsec_otp_data58::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA58 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data58::W](bsec_otp_data58::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA58 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data58;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data59](bsec_otp_data59) module"]
pub type BSEC_OTP_DATA59 = crate::Reg<u32, _BSEC_OTP_DATA59>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA59;
#[doc = "`read()` method returns [bsec_otp_data59::R](bsec_otp_data59::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA59 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data59::W](bsec_otp_data59::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA59 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data59;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data60](bsec_otp_data60) module"]
pub type BSEC_OTP_DATA60 = crate::Reg<u32, _BSEC_OTP_DATA60>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA60;
#[doc = "`read()` method returns [bsec_otp_data60::R](bsec_otp_data60::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA60 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data60::W](bsec_otp_data60::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA60 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data60;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data61](bsec_otp_data61) module"]
pub type BSEC_OTP_DATA61 = crate::Reg<u32, _BSEC_OTP_DATA61>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA61;
#[doc = "`read()` method returns [bsec_otp_data61::R](bsec_otp_data61::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA61 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data61::W](bsec_otp_data61::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA61 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data61;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data62](bsec_otp_data62) module"]
pub type BSEC_OTP_DATA62 = crate::Reg<u32, _BSEC_OTP_DATA62>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA62;
#[doc = "`read()` method returns [bsec_otp_data62::R](bsec_otp_data62::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA62 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data62::W](bsec_otp_data62::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA62 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data62;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data63](bsec_otp_data63) module"]
pub type BSEC_OTP_DATA63 = crate::Reg<u32, _BSEC_OTP_DATA63>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA63;
#[doc = "`read()` method returns [bsec_otp_data63::R](bsec_otp_data63::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA63 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data63::W](bsec_otp_data63::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA63 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data63;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data64](bsec_otp_data64) module"]
pub type BSEC_OTP_DATA64 = crate::Reg<u32, _BSEC_OTP_DATA64>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA64;
#[doc = "`read()` method returns [bsec_otp_data64::R](bsec_otp_data64::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA64 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data64::W](bsec_otp_data64::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA64 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data64;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data65](bsec_otp_data65) module"]
pub type BSEC_OTP_DATA65 = crate::Reg<u32, _BSEC_OTP_DATA65>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA65;
#[doc = "`read()` method returns [bsec_otp_data65::R](bsec_otp_data65::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA65 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data65::W](bsec_otp_data65::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA65 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data65;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data66](bsec_otp_data66) module"]
pub type BSEC_OTP_DATA66 = crate::Reg<u32, _BSEC_OTP_DATA66>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA66;
#[doc = "`read()` method returns [bsec_otp_data66::R](bsec_otp_data66::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA66 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data66::W](bsec_otp_data66::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA66 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data66;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data67](bsec_otp_data67) module"]
pub type BSEC_OTP_DATA67 = crate::Reg<u32, _BSEC_OTP_DATA67>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA67;
#[doc = "`read()` method returns [bsec_otp_data67::R](bsec_otp_data67::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA67 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data67::W](bsec_otp_data67::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA67 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data67;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data68](bsec_otp_data68) module"]
pub type BSEC_OTP_DATA68 = crate::Reg<u32, _BSEC_OTP_DATA68>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA68;
#[doc = "`read()` method returns [bsec_otp_data68::R](bsec_otp_data68::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA68 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data68::W](bsec_otp_data68::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA68 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data68;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data69](bsec_otp_data69) module"]
pub type BSEC_OTP_DATA69 = crate::Reg<u32, _BSEC_OTP_DATA69>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA69;
#[doc = "`read()` method returns [bsec_otp_data69::R](bsec_otp_data69::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA69 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data69::W](bsec_otp_data69::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA69 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data69;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data70](bsec_otp_data70) module"]
pub type BSEC_OTP_DATA70 = crate::Reg<u32, _BSEC_OTP_DATA70>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA70;
#[doc = "`read()` method returns [bsec_otp_data70::R](bsec_otp_data70::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA70 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data70::W](bsec_otp_data70::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA70 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data70;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data71](bsec_otp_data71) module"]
pub type BSEC_OTP_DATA71 = crate::Reg<u32, _BSEC_OTP_DATA71>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA71;
#[doc = "`read()` method returns [bsec_otp_data71::R](bsec_otp_data71::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA71 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data71::W](bsec_otp_data71::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA71 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data71;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data72](bsec_otp_data72) module"]
pub type BSEC_OTP_DATA72 = crate::Reg<u32, _BSEC_OTP_DATA72>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA72;
#[doc = "`read()` method returns [bsec_otp_data72::R](bsec_otp_data72::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA72 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data72::W](bsec_otp_data72::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA72 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data72;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data73](bsec_otp_data73) module"]
pub type BSEC_OTP_DATA73 = crate::Reg<u32, _BSEC_OTP_DATA73>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA73;
#[doc = "`read()` method returns [bsec_otp_data73::R](bsec_otp_data73::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA73 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data73::W](bsec_otp_data73::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA73 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data73;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data74](bsec_otp_data74) module"]
pub type BSEC_OTP_DATA74 = crate::Reg<u32, _BSEC_OTP_DATA74>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA74;
#[doc = "`read()` method returns [bsec_otp_data74::R](bsec_otp_data74::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA74 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data74::W](bsec_otp_data74::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA74 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data74;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data75](bsec_otp_data75) module"]
pub type BSEC_OTP_DATA75 = crate::Reg<u32, _BSEC_OTP_DATA75>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA75;
#[doc = "`read()` method returns [bsec_otp_data75::R](bsec_otp_data75::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA75 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data75::W](bsec_otp_data75::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA75 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data75;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data76](bsec_otp_data76) module"]
pub type BSEC_OTP_DATA76 = crate::Reg<u32, _BSEC_OTP_DATA76>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA76;
#[doc = "`read()` method returns [bsec_otp_data76::R](bsec_otp_data76::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA76 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data76::W](bsec_otp_data76::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA76 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data76;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data77](bsec_otp_data77) module"]
pub type BSEC_OTP_DATA77 = crate::Reg<u32, _BSEC_OTP_DATA77>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA77;
#[doc = "`read()` method returns [bsec_otp_data77::R](bsec_otp_data77::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA77 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data77::W](bsec_otp_data77::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA77 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data77;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data78](bsec_otp_data78) module"]
pub type BSEC_OTP_DATA78 = crate::Reg<u32, _BSEC_OTP_DATA78>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA78;
#[doc = "`read()` method returns [bsec_otp_data78::R](bsec_otp_data78::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA78 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data78::W](bsec_otp_data78::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA78 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data78;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data79](bsec_otp_data79) module"]
pub type BSEC_OTP_DATA79 = crate::Reg<u32, _BSEC_OTP_DATA79>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA79;
#[doc = "`read()` method returns [bsec_otp_data79::R](bsec_otp_data79::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA79 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data79::W](bsec_otp_data79::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA79 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data79;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data80](bsec_otp_data80) module"]
pub type BSEC_OTP_DATA80 = crate::Reg<u32, _BSEC_OTP_DATA80>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA80;
#[doc = "`read()` method returns [bsec_otp_data80::R](bsec_otp_data80::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA80 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data80::W](bsec_otp_data80::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA80 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data80;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data81](bsec_otp_data81) module"]
pub type BSEC_OTP_DATA81 = crate::Reg<u32, _BSEC_OTP_DATA81>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA81;
#[doc = "`read()` method returns [bsec_otp_data81::R](bsec_otp_data81::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA81 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data81::W](bsec_otp_data81::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA81 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data81;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data82](bsec_otp_data82) module"]
pub type BSEC_OTP_DATA82 = crate::Reg<u32, _BSEC_OTP_DATA82>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA82;
#[doc = "`read()` method returns [bsec_otp_data82::R](bsec_otp_data82::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA82 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data82::W](bsec_otp_data82::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA82 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data82;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data83](bsec_otp_data83) module"]
pub type BSEC_OTP_DATA83 = crate::Reg<u32, _BSEC_OTP_DATA83>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA83;
#[doc = "`read()` method returns [bsec_otp_data83::R](bsec_otp_data83::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA83 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data83::W](bsec_otp_data83::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA83 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data83;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data84](bsec_otp_data84) module"]
pub type BSEC_OTP_DATA84 = crate::Reg<u32, _BSEC_OTP_DATA84>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA84;
#[doc = "`read()` method returns [bsec_otp_data84::R](bsec_otp_data84::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA84 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data84::W](bsec_otp_data84::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA84 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data84;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data85](bsec_otp_data85) module"]
pub type BSEC_OTP_DATA85 = crate::Reg<u32, _BSEC_OTP_DATA85>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA85;
#[doc = "`read()` method returns [bsec_otp_data85::R](bsec_otp_data85::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA85 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data85::W](bsec_otp_data85::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA85 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data85;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data86](bsec_otp_data86) module"]
pub type BSEC_OTP_DATA86 = crate::Reg<u32, _BSEC_OTP_DATA86>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA86;
#[doc = "`read()` method returns [bsec_otp_data86::R](bsec_otp_data86::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA86 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data86::W](bsec_otp_data86::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA86 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data86;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data87](bsec_otp_data87) module"]
pub type BSEC_OTP_DATA87 = crate::Reg<u32, _BSEC_OTP_DATA87>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA87;
#[doc = "`read()` method returns [bsec_otp_data87::R](bsec_otp_data87::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA87 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data87::W](bsec_otp_data87::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA87 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data87;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data88](bsec_otp_data88) module"]
pub type BSEC_OTP_DATA88 = crate::Reg<u32, _BSEC_OTP_DATA88>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA88;
#[doc = "`read()` method returns [bsec_otp_data88::R](bsec_otp_data88::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA88 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data88::W](bsec_otp_data88::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA88 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data88;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data89](bsec_otp_data89) module"]
pub type BSEC_OTP_DATA89 = crate::Reg<u32, _BSEC_OTP_DATA89>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA89;
#[doc = "`read()` method returns [bsec_otp_data89::R](bsec_otp_data89::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA89 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data89::W](bsec_otp_data89::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA89 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data89;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data90](bsec_otp_data90) module"]
pub type BSEC_OTP_DATA90 = crate::Reg<u32, _BSEC_OTP_DATA90>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA90;
#[doc = "`read()` method returns [bsec_otp_data90::R](bsec_otp_data90::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA90 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data90::W](bsec_otp_data90::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA90 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data90;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data91](bsec_otp_data91) module"]
pub type BSEC_OTP_DATA91 = crate::Reg<u32, _BSEC_OTP_DATA91>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA91;
#[doc = "`read()` method returns [bsec_otp_data91::R](bsec_otp_data91::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA91 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data91::W](bsec_otp_data91::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA91 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data91;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data92](bsec_otp_data92) module"]
pub type BSEC_OTP_DATA92 = crate::Reg<u32, _BSEC_OTP_DATA92>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA92;
#[doc = "`read()` method returns [bsec_otp_data92::R](bsec_otp_data92::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA92 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data92::W](bsec_otp_data92::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA92 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data92;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data93](bsec_otp_data93) module"]
pub type BSEC_OTP_DATA93 = crate::Reg<u32, _BSEC_OTP_DATA93>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA93;
#[doc = "`read()` method returns [bsec_otp_data93::R](bsec_otp_data93::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA93 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data93::W](bsec_otp_data93::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA93 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data93;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data94](bsec_otp_data94) module"]
pub type BSEC_OTP_DATA94 = crate::Reg<u32, _BSEC_OTP_DATA94>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA94;
#[doc = "`read()` method returns [bsec_otp_data94::R](bsec_otp_data94::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA94 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data94::W](bsec_otp_data94::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA94 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data94;
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data95](bsec_otp_data95) module"]
pub type BSEC_OTP_DATA95 = crate::Reg<u32, _BSEC_OTP_DATA95>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_OTP_DATA95;
#[doc = "`read()` method returns [bsec_otp_data95::R](bsec_otp_data95::R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA95 {}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data95::W](bsec_otp_data95::W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA95 {}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode."]
pub mod bsec_otp_data95;
#[doc = "BSEC hardware configuration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_hwcfgr](bsec_hwcfgr) module"]
pub type BSEC_HWCFGR = crate::Reg<u32, _BSEC_HWCFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_HWCFGR;
#[doc = "`read()` method returns [bsec_hwcfgr::R](bsec_hwcfgr::R) reader structure"]
impl crate::Readable for BSEC_HWCFGR {}
#[doc = "BSEC hardware configuration register"]
pub mod bsec_hwcfgr;
#[doc = "BSEC version register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_verr](bsec_verr) module"]
pub type BSEC_VERR = crate::Reg<u32, _BSEC_VERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_VERR;
#[doc = "`read()` method returns [bsec_verr::R](bsec_verr::R) reader structure"]
impl crate::Readable for BSEC_VERR {}
#[doc = "BSEC version register"]
pub mod bsec_verr;
#[doc = "BSEC identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_ipidr](bsec_ipidr) module"]
pub type BSEC_IPIDR = crate::Reg<u32, _BSEC_IPIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_IPIDR;
#[doc = "`read()` method returns [bsec_ipidr::R](bsec_ipidr::R) reader structure"]
impl crate::Readable for BSEC_IPIDR {}
#[doc = "BSEC identification register"]
pub mod bsec_ipidr;
#[doc = "BSEC size identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_sidr](bsec_sidr) module"]
pub type BSEC_SIDR = crate::Reg<u32, _BSEC_SIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSEC_SIDR;
#[doc = "`read()` method returns [bsec_sidr::R](bsec_sidr::R) reader structure"]
impl crate::Readable for BSEC_SIDR {}
#[doc = "BSEC size identification register"]
pub mod bsec_sidr;
