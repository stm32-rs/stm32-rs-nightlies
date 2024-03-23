#[doc = "Register `DCR1` reader"]
pub type R = crate::R<DCR1rs>;
#[doc = "Register `DCR1` writer"]
pub type W = crate::W<DCR1rs>;
#[doc = "Field `CKMODE` reader - Mode 0/Mode 3 This bit indicates the level taken by the CLK between commands (when NCS = 1)."]
pub type CKMODE_R = crate::BitReader;
#[doc = "Field `CKMODE` writer - Mode 0/Mode 3 This bit indicates the level taken by the CLK between commands (when NCS = 1)."]
pub type CKMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRCK` reader - Free running clock This bit configures the free running clock."]
pub type FRCK_R = crate::BitReader;
#[doc = "Field `FRCK` writer - Free running clock This bit configures the free running clock."]
pub type FRCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLYBYP` reader - Delay block bypass"]
pub type DLYBYP_R = crate::BitReader;
#[doc = "Field `DLYBYP` writer - Delay block bypass"]
pub type DLYBYP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSHT` reader - Chip-select high time CSHT + 1 defines the minimum number of CLK cycles where the chip-select (NCS) must remain high between commands issued to the external device. ..."]
pub type CSHT_R = crate::FieldReader;
#[doc = "Field `CSHT` writer - Chip-select high time CSHT + 1 defines the minimum number of CLK cycles where the chip-select (NCS) must remain high between commands issued to the external device. ..."]
pub type CSHT_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DEVSIZE` reader - Device size This field defines the size of the external device using the following formula: Number of bytes in device = 2\\[DEVSIZE+1\\]. DEVSIZE+1 is effectively the number of address bits required to address the external device. The device capacity can be up to 4 Gbytes (addressed using 32-bits) in Indirect mode, but the addressable space in Memory-mapped mode is limited to 256 Mbytes. In Regular-command protocol, if DMM = 1, DEVSIZE\\[4:0\\]
indicates the total capacity of the two devices together."]
pub type DEVSIZE_R = crate::FieldReader;
#[doc = "Field `DEVSIZE` writer - Device size This field defines the size of the external device using the following formula: Number of bytes in device = 2\\[DEVSIZE+1\\]. DEVSIZE+1 is effectively the number of address bits required to address the external device. The device capacity can be up to 4 Gbytes (addressed using 32-bits) in Indirect mode, but the addressable space in Memory-mapped mode is limited to 256 Mbytes. In Regular-command protocol, if DMM = 1, DEVSIZE\\[4:0\\]
indicates the total capacity of the two devices together."]
pub type DEVSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `MTYP` reader - Memory type This bit indicates the type of memory to be supported. Note: In this mode, DQS signal polarity is inverted with respect to the memory clock signal. This is the default value and care must be taken to change MTYP\\[2:0\\]
for memories different from Micron. Others: Reserved"]
pub type MTYP_R = crate::FieldReader;
#[doc = "Field `MTYP` writer - Memory type This bit indicates the type of memory to be supported. Note: In this mode, DQS signal polarity is inverted with respect to the memory clock signal. This is the default value and care must be taken to change MTYP\\[2:0\\]
for memories different from Micron. Others: Reserved"]
pub type MTYP_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Mode 0/Mode 3 This bit indicates the level taken by the CLK between commands (when NCS = 1)."]
    #[inline(always)]
    pub fn ckmode(&self) -> CKMODE_R {
        CKMODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Free running clock This bit configures the free running clock."]
    #[inline(always)]
    pub fn frck(&self) -> FRCK_R {
        FRCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Delay block bypass"]
    #[inline(always)]
    pub fn dlybyp(&self) -> DLYBYP_R {
        DLYBYP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:13 - Chip-select high time CSHT + 1 defines the minimum number of CLK cycles where the chip-select (NCS) must remain high between commands issued to the external device. ..."]
    #[inline(always)]
    pub fn csht(&self) -> CSHT_R {
        CSHT_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:20 - Device size This field defines the size of the external device using the following formula: Number of bytes in device = 2\\[DEVSIZE+1\\]. DEVSIZE+1 is effectively the number of address bits required to address the external device. The device capacity can be up to 4 Gbytes (addressed using 32-bits) in Indirect mode, but the addressable space in Memory-mapped mode is limited to 256 Mbytes. In Regular-command protocol, if DMM = 1, DEVSIZE\\[4:0\\]
indicates the total capacity of the two devices together."]
    #[inline(always)]
    pub fn devsize(&self) -> DEVSIZE_R {
        DEVSIZE_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:26 - Memory type This bit indicates the type of memory to be supported. Note: In this mode, DQS signal polarity is inverted with respect to the memory clock signal. This is the default value and care must be taken to change MTYP\\[2:0\\]
for memories different from Micron. Others: Reserved"]
    #[inline(always)]
    pub fn mtyp(&self) -> MTYP_R {
        MTYP_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Mode 0/Mode 3 This bit indicates the level taken by the CLK between commands (when NCS = 1)."]
    #[inline(always)]
    #[must_use]
    pub fn ckmode(&mut self) -> CKMODE_W<DCR1rs> {
        CKMODE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Free running clock This bit configures the free running clock."]
    #[inline(always)]
    #[must_use]
    pub fn frck(&mut self) -> FRCK_W<DCR1rs> {
        FRCK_W::new(self, 1)
    }
    #[doc = "Bit 3 - Delay block bypass"]
    #[inline(always)]
    #[must_use]
    pub fn dlybyp(&mut self) -> DLYBYP_W<DCR1rs> {
        DLYBYP_W::new(self, 3)
    }
    #[doc = "Bits 8:13 - Chip-select high time CSHT + 1 defines the minimum number of CLK cycles where the chip-select (NCS) must remain high between commands issued to the external device. ..."]
    #[inline(always)]
    #[must_use]
    pub fn csht(&mut self) -> CSHT_W<DCR1rs> {
        CSHT_W::new(self, 8)
    }
    #[doc = "Bits 16:20 - Device size This field defines the size of the external device using the following formula: Number of bytes in device = 2\\[DEVSIZE+1\\]. DEVSIZE+1 is effectively the number of address bits required to address the external device. The device capacity can be up to 4 Gbytes (addressed using 32-bits) in Indirect mode, but the addressable space in Memory-mapped mode is limited to 256 Mbytes. In Regular-command protocol, if DMM = 1, DEVSIZE\\[4:0\\]
indicates the total capacity of the two devices together."]
    #[inline(always)]
    #[must_use]
    pub fn devsize(&mut self) -> DEVSIZE_W<DCR1rs> {
        DEVSIZE_W::new(self, 16)
    }
    #[doc = "Bits 24:26 - Memory type This bit indicates the type of memory to be supported. Note: In this mode, DQS signal polarity is inverted with respect to the memory clock signal. This is the default value and care must be taken to change MTYP\\[2:0\\]
for memories different from Micron. Others: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn mtyp(&mut self) -> MTYP_W<DCR1rs> {
        MTYP_W::new(self, 24)
    }
}
#[doc = "OCTOSPI device configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCR1rs;
impl crate::RegisterSpec for DCR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcr1::R`](R) reader structure"]
impl crate::Readable for DCR1rs {}
#[doc = "`write(|w| ..)` method takes [`dcr1::W`](W) writer structure"]
impl crate::Writable for DCR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCR1 to value 0"]
impl crate::Resettable for DCR1rs {
    const RESET_VALUE: u32 = 0;
}
