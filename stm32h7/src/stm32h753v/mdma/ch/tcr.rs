///Register `TCR` reader
pub type R = crate::R<TCRrs>;
///Register `TCR` writer
pub type W = crate::W<TCRrs>;
///Field `SINC` reader - Source increment mode These bits are set and cleared by software. These bits are protected and can be written only if EN is 0 Note: When source is AHB (SBUS=1), SINC = 00 is forbidden. In Linked List Mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
pub type SINC_R = crate::FieldReader;
///Field `SINC` writer - Source increment mode These bits are set and cleared by software. These bits are protected and can be written only if EN is 0 Note: When source is AHB (SBUS=1), SINC = 00 is forbidden. In Linked List Mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
pub type SINC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DINC` reader - Destination increment mode These bits are set and cleared by software. These bits are protected and can be written only if EN is 0 Note: When destination is AHB (DBUS=1), DINC = 00 is forbidden.
pub type DINC_R = crate::FieldReader;
///Field `DINC` writer - Destination increment mode These bits are set and cleared by software. These bits are protected and can be written only if EN is 0 Note: When destination is AHB (DBUS=1), DINC = 00 is forbidden.
pub type DINC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SSIZE` reader - Source data size These bits are set and cleared by software. These bits are protected and can be written only if EN is 0 Note: If a value of 11 is programmed for the TCM access/AHB port, a transfer error will occur (TEIF bit set) If SINCOS &lt; SSIZE and SINC &#8800; 00, the result will be unpredictable. Note: SSIZE = 11 (double-word) is forbidden when source is TCM/AHB bus (SBUS=1).
pub type SSIZE_R = crate::FieldReader;
///Field `SSIZE` writer - Source data size These bits are set and cleared by software. These bits are protected and can be written only if EN is 0 Note: If a value of 11 is programmed for the TCM access/AHB port, a transfer error will occur (TEIF bit set) If SINCOS &lt; SSIZE and SINC &#8800; 00, the result will be unpredictable. Note: SSIZE = 11 (double-word) is forbidden when source is TCM/AHB bus (SBUS=1).
pub type SSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DSIZE` reader - Destination data size These bits are set and cleared by software. These bits are protected and can be written only if EN is 0. Note: If a value of 11 is programmed for the TCM access/AHB port, a transfer error will occur (TEIF bit set) If DINCOS &lt; DSIZE and DINC &#8800; 00, the result will be unpredictable. Note: DSIZE = 11 (double-word) is forbidden when destination is TCM/AHB bus (DBUS=1).
pub type DSIZE_R = crate::FieldReader;
///Field `DSIZE` writer - Destination data size These bits are set and cleared by software. These bits are protected and can be written only if EN is 0. Note: If a value of 11 is programmed for the TCM access/AHB port, a transfer error will occur (TEIF bit set) If DINCOS &lt; DSIZE and DINC &#8800; 00, the result will be unpredictable. Note: DSIZE = 11 (double-word) is forbidden when destination is TCM/AHB bus (DBUS=1).
pub type DSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SINCOS` reader - source increment offset size
pub type SINCOS_R = crate::FieldReader;
///Field `SINCOS` writer - source increment offset size
pub type SINCOS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DINCOS` reader - Destination increment offset
pub type DINCOS_R = crate::FieldReader;
///Field `DINCOS` writer - Destination increment offset
pub type DINCOS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SBURST` reader - source burst transfer configuration
pub type SBURST_R = crate::FieldReader;
///Field `SBURST` writer - source burst transfer configuration
pub type SBURST_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DBURST` reader - Destination burst transfer configuration
pub type DBURST_R = crate::FieldReader;
///Field `DBURST` writer - Destination burst transfer configuration
pub type DBURST_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `TLEN` reader - buffer transfer lengh
pub type TLEN_R = crate::FieldReader;
///Field `TLEN` writer - buffer transfer lengh
pub type TLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `PKE` reader - PacK Enable These bit is set and cleared by software. If the Source Size is smaller than the destination, it will be padded according to the PAM value. If the Source data size is larger than the destination one, it will be truncated. The alignment will be done according to the PAM\[0\] value. This bit is protected and can be written only if EN is 0
pub type PKE_R = crate::BitReader;
///Field `PKE` writer - PacK Enable These bit is set and cleared by software. If the Source Size is smaller than the destination, it will be padded according to the PAM value. If the Source data size is larger than the destination one, it will be truncated. The alignment will be done according to the PAM\[0\] value. This bit is protected and can be written only if EN is 0
pub type PKE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PAM` reader - Padding/Alignement Mode These bits are set and cleared by software. Case 1: Source data size smaller than destination data size - 3 options are valid. Case 2: Source data size larger than destination data size. The remainder part is discarded. When PKE = 1 or DSIZE=SSIZE, these bits are ignored. These bits are protected and can be written only if EN is 0
pub type PAM_R = crate::FieldReader;
///Field `PAM` writer - Padding/Alignement Mode These bits are set and cleared by software. Case 1: Source data size smaller than destination data size - 3 options are valid. Case 2: Source data size larger than destination data size. The remainder part is discarded. When PKE = 1 or DSIZE=SSIZE, these bits are ignored. These bits are protected and can be written only if EN is 0
pub type PAM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TRGM` reader - Trigger Mode These bits are set and cleared by software. Note: If TRGM is 11 for the current block, all the values loaded at the end of the current block through the linked list mechanism must keep the same value (TRGM=11) and the same SWRM value, otherwise the result is undefined. These bits are protected and can be written only if EN is 0.
pub type TRGM_R = crate::FieldReader;
///Field `TRGM` writer - Trigger Mode These bits are set and cleared by software. Note: If TRGM is 11 for the current block, all the values loaded at the end of the current block through the linked list mechanism must keep the same value (TRGM=11) and the same SWRM value, otherwise the result is undefined. These bits are protected and can be written only if EN is 0.
pub type TRGM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SWRM` reader - SW Request Mode This bit is set and cleared by software. If a HW or SW request is currently active, the bit change will be delayed until the current transfer is completed. If the CxMAR contains a valid address, the CxMDR value will also be written @ CxMAR address. This bit is protected and can be written only if EN is 0.
pub type SWRM_R = crate::BitReader;
///Field `SWRM` writer - SW Request Mode This bit is set and cleared by software. If a HW or SW request is currently active, the bit change will be delayed until the current transfer is completed. If the CxMAR contains a valid address, the CxMDR value will also be written @ CxMAR address. This bit is protected and can be written only if EN is 0.
pub type SWRM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BWM` reader - Bufferable Write Mode This bit is set and cleared by software. This bit is protected and can be written only if EN is 0. Note: All MDMA destination accesses are non-cacheable.
pub type BWM_R = crate::BitReader;
///Field `BWM` writer - Bufferable Write Mode This bit is set and cleared by software. This bit is protected and can be written only if EN is 0. Note: All MDMA destination accesses are non-cacheable.
pub type BWM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - Source increment mode These bits are set and cleared by software. These bits are protected and can be written only if EN is 0 Note: When source is AHB (SBUS=1), SINC = 00 is forbidden. In Linked List Mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
    #[inline(always)]
    pub fn sinc(&self) -> SINC_R {
        SINC_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - Destination increment mode These bits are set and cleared by software. These bits are protected and can be written only if EN is 0 Note: When destination is AHB (DBUS=1), DINC = 00 is forbidden.
    #[inline(always)]
    pub fn dinc(&self) -> DINC_R {
        DINC_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - Source data size These bits are set and cleared by software. These bits are protected and can be written only if EN is 0 Note: If a value of 11 is programmed for the TCM access/AHB port, a transfer error will occur (TEIF bit set) If SINCOS &lt; SSIZE and SINC &#8800; 00, the result will be unpredictable. Note: SSIZE = 11 (double-word) is forbidden when source is TCM/AHB bus (SBUS=1).
    #[inline(always)]
    pub fn ssize(&self) -> SSIZE_R {
        SSIZE_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - Destination data size These bits are set and cleared by software. These bits are protected and can be written only if EN is 0. Note: If a value of 11 is programmed for the TCM access/AHB port, a transfer error will occur (TEIF bit set) If DINCOS &lt; DSIZE and DINC &#8800; 00, the result will be unpredictable. Note: DSIZE = 11 (double-word) is forbidden when destination is TCM/AHB bus (DBUS=1).
    #[inline(always)]
    pub fn dsize(&self) -> DSIZE_R {
        DSIZE_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - source increment offset size
    #[inline(always)]
    pub fn sincos(&self) -> SINCOS_R {
        SINCOS_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - Destination increment offset
    #[inline(always)]
    pub fn dincos(&self) -> DINCOS_R {
        DINCOS_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:14 - source burst transfer configuration
    #[inline(always)]
    pub fn sburst(&self) -> SBURST_R {
        SBURST_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bits 15:17 - Destination burst transfer configuration
    #[inline(always)]
    pub fn dburst(&self) -> DBURST_R {
        DBURST_R::new(((self.bits >> 15) & 7) as u8)
    }
    ///Bits 18:24 - buffer transfer lengh
    #[inline(always)]
    pub fn tlen(&self) -> TLEN_R {
        TLEN_R::new(((self.bits >> 18) & 0x7f) as u8)
    }
    ///Bit 25 - PacK Enable These bit is set and cleared by software. If the Source Size is smaller than the destination, it will be padded according to the PAM value. If the Source data size is larger than the destination one, it will be truncated. The alignment will be done according to the PAM\[0\] value. This bit is protected and can be written only if EN is 0
    #[inline(always)]
    pub fn pke(&self) -> PKE_R {
        PKE_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bits 26:27 - Padding/Alignement Mode These bits are set and cleared by software. Case 1: Source data size smaller than destination data size - 3 options are valid. Case 2: Source data size larger than destination data size. The remainder part is discarded. When PKE = 1 or DSIZE=SSIZE, these bits are ignored. These bits are protected and can be written only if EN is 0
    #[inline(always)]
    pub fn pam(&self) -> PAM_R {
        PAM_R::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bits 28:29 - Trigger Mode These bits are set and cleared by software. Note: If TRGM is 11 for the current block, all the values loaded at the end of the current block through the linked list mechanism must keep the same value (TRGM=11) and the same SWRM value, otherwise the result is undefined. These bits are protected and can be written only if EN is 0.
    #[inline(always)]
    pub fn trgm(&self) -> TRGM_R {
        TRGM_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bit 30 - SW Request Mode This bit is set and cleared by software. If a HW or SW request is currently active, the bit change will be delayed until the current transfer is completed. If the CxMAR contains a valid address, the CxMDR value will also be written @ CxMAR address. This bit is protected and can be written only if EN is 0.
    #[inline(always)]
    pub fn swrm(&self) -> SWRM_R {
        SWRM_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Bufferable Write Mode This bit is set and cleared by software. This bit is protected and can be written only if EN is 0. Note: All MDMA destination accesses are non-cacheable.
    #[inline(always)]
    pub fn bwm(&self) -> BWM_R {
        BWM_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TCR")
            .field("sinc", &self.sinc())
            .field("dinc", &self.dinc())
            .field("ssize", &self.ssize())
            .field("dsize", &self.dsize())
            .field("sincos", &self.sincos())
            .field("dincos", &self.dincos())
            .field("sburst", &self.sburst())
            .field("dburst", &self.dburst())
            .field("tlen", &self.tlen())
            .field("pke", &self.pke())
            .field("pam", &self.pam())
            .field("trgm", &self.trgm())
            .field("swrm", &self.swrm())
            .field("bwm", &self.bwm())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Source increment mode These bits are set and cleared by software. These bits are protected and can be written only if EN is 0 Note: When source is AHB (SBUS=1), SINC = 00 is forbidden. In Linked List Mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
    #[inline(always)]
    pub fn sinc(&mut self) -> SINC_W<TCRrs> {
        SINC_W::new(self, 0)
    }
    ///Bits 2:3 - Destination increment mode These bits are set and cleared by software. These bits are protected and can be written only if EN is 0 Note: When destination is AHB (DBUS=1), DINC = 00 is forbidden.
    #[inline(always)]
    pub fn dinc(&mut self) -> DINC_W<TCRrs> {
        DINC_W::new(self, 2)
    }
    ///Bits 4:5 - Source data size These bits are set and cleared by software. These bits are protected and can be written only if EN is 0 Note: If a value of 11 is programmed for the TCM access/AHB port, a transfer error will occur (TEIF bit set) If SINCOS &lt; SSIZE and SINC &#8800; 00, the result will be unpredictable. Note: SSIZE = 11 (double-word) is forbidden when source is TCM/AHB bus (SBUS=1).
    #[inline(always)]
    pub fn ssize(&mut self) -> SSIZE_W<TCRrs> {
        SSIZE_W::new(self, 4)
    }
    ///Bits 6:7 - Destination data size These bits are set and cleared by software. These bits are protected and can be written only if EN is 0. Note: If a value of 11 is programmed for the TCM access/AHB port, a transfer error will occur (TEIF bit set) If DINCOS &lt; DSIZE and DINC &#8800; 00, the result will be unpredictable. Note: DSIZE = 11 (double-word) is forbidden when destination is TCM/AHB bus (DBUS=1).
    #[inline(always)]
    pub fn dsize(&mut self) -> DSIZE_W<TCRrs> {
        DSIZE_W::new(self, 6)
    }
    ///Bits 8:9 - source increment offset size
    #[inline(always)]
    pub fn sincos(&mut self) -> SINCOS_W<TCRrs> {
        SINCOS_W::new(self, 8)
    }
    ///Bits 10:11 - Destination increment offset
    #[inline(always)]
    pub fn dincos(&mut self) -> DINCOS_W<TCRrs> {
        DINCOS_W::new(self, 10)
    }
    ///Bits 12:14 - source burst transfer configuration
    #[inline(always)]
    pub fn sburst(&mut self) -> SBURST_W<TCRrs> {
        SBURST_W::new(self, 12)
    }
    ///Bits 15:17 - Destination burst transfer configuration
    #[inline(always)]
    pub fn dburst(&mut self) -> DBURST_W<TCRrs> {
        DBURST_W::new(self, 15)
    }
    ///Bits 18:24 - buffer transfer lengh
    #[inline(always)]
    pub fn tlen(&mut self) -> TLEN_W<TCRrs> {
        TLEN_W::new(self, 18)
    }
    ///Bit 25 - PacK Enable These bit is set and cleared by software. If the Source Size is smaller than the destination, it will be padded according to the PAM value. If the Source data size is larger than the destination one, it will be truncated. The alignment will be done according to the PAM\[0\] value. This bit is protected and can be written only if EN is 0
    #[inline(always)]
    pub fn pke(&mut self) -> PKE_W<TCRrs> {
        PKE_W::new(self, 25)
    }
    ///Bits 26:27 - Padding/Alignement Mode These bits are set and cleared by software. Case 1: Source data size smaller than destination data size - 3 options are valid. Case 2: Source data size larger than destination data size. The remainder part is discarded. When PKE = 1 or DSIZE=SSIZE, these bits are ignored. These bits are protected and can be written only if EN is 0
    #[inline(always)]
    pub fn pam(&mut self) -> PAM_W<TCRrs> {
        PAM_W::new(self, 26)
    }
    ///Bits 28:29 - Trigger Mode These bits are set and cleared by software. Note: If TRGM is 11 for the current block, all the values loaded at the end of the current block through the linked list mechanism must keep the same value (TRGM=11) and the same SWRM value, otherwise the result is undefined. These bits are protected and can be written only if EN is 0.
    #[inline(always)]
    pub fn trgm(&mut self) -> TRGM_W<TCRrs> {
        TRGM_W::new(self, 28)
    }
    ///Bit 30 - SW Request Mode This bit is set and cleared by software. If a HW or SW request is currently active, the bit change will be delayed until the current transfer is completed. If the CxMAR contains a valid address, the CxMDR value will also be written @ CxMAR address. This bit is protected and can be written only if EN is 0.
    #[inline(always)]
    pub fn swrm(&mut self) -> SWRM_W<TCRrs> {
        SWRM_W::new(self, 30)
    }
    ///Bit 31 - Bufferable Write Mode This bit is set and cleared by software. This bit is protected and can be written only if EN is 0. Note: All MDMA destination accesses are non-cacheable.
    #[inline(always)]
    pub fn bwm(&mut self) -> BWM_W<TCRrs> {
        BWM_W::new(self, 31)
    }
}
/**This register is used to configure the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`tcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TCRrs;
impl crate::RegisterSpec for TCRrs {
    type Ux = u32;
}
///`read()` method returns [`tcr::R`](R) reader structure
impl crate::Readable for TCRrs {}
///`write(|w| ..)` method takes [`tcr::W`](W) writer structure
impl crate::Writable for TCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TCR to value 0
impl crate::Resettable for TCRrs {}
