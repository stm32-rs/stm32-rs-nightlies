#[doc = "Register `FCR4` writer"]
pub type W = crate::W<FCR4rs>;
#[doc = "Field `CGPDMA1F` writer - clear the illegal access flag for GPDMA1"]
pub type CGPDMA1F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFLASH_REGF` writer - clear the illegal access flag for FLASH registers"]
pub type CFLASH_REGF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFLASHF` writer - clear the illegal access flag for FLASH memory"]
pub type CFLASHF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COTFDEC1F` writer - clear the illegal access flag for OTFDEC1"]
pub type COTFDEC1F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTZSC1F` writer - clear the illegal access flag for GTZC1 TZSC registers"]
pub type CTZSC1F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTZIC1F` writer - clear the illegal access flag for GTZC1 TZIC registers"]
pub type CTZIC1F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COCTOSPI1_MEMF` writer - clear the illegal access flag for MPCWM1 (OCTOSPI1) memory bank"]
pub type COCTOSPI1_MEMF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBKPSRAMF` writer - clear the illegal access flag for MPCWM3 (BKPSRAM) memory bank"]
pub type CBKPSRAMF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHSPI1_MEMF` writer - clear the illegal access flag for HSPI1 memory bank"]
pub type CHSPI1_MEMF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSRAM1F` writer - clear the illegal access flag for SRAM1"]
pub type CSRAM1F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPCBB1_REGF` writer - clear the illegal access flag for MPCBB1 registers"]
pub type CMPCBB1_REGF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSRAM2F` writer - clear the illegal access flag for SRAM2"]
pub type CSRAM2F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPCBB2_REGF` writer - clear the illegal access flag for MPCBB2 registers"]
pub type CMPCBB2_REGF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSRAM5F` writer - clear the illegal access flag for SRAM5"]
pub type CSRAM5F_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - clear the illegal access flag for GPDMA1"]
    #[inline(always)]
    #[must_use]
    pub fn cgpdma1f(&mut self) -> CGPDMA1F_W<FCR4rs> {
        CGPDMA1F_W::new(self, 0)
    }
    #[doc = "Bit 1 - clear the illegal access flag for FLASH registers"]
    #[inline(always)]
    #[must_use]
    pub fn cflash_regf(&mut self) -> CFLASH_REGF_W<FCR4rs> {
        CFLASH_REGF_W::new(self, 1)
    }
    #[doc = "Bit 2 - clear the illegal access flag for FLASH memory"]
    #[inline(always)]
    #[must_use]
    pub fn cflashf(&mut self) -> CFLASHF_W<FCR4rs> {
        CFLASHF_W::new(self, 2)
    }
    #[doc = "Bit 3 - clear the illegal access flag for OTFDEC1"]
    #[inline(always)]
    #[must_use]
    pub fn cotfdec1f(&mut self) -> COTFDEC1F_W<FCR4rs> {
        COTFDEC1F_W::new(self, 3)
    }
    #[doc = "Bit 14 - clear the illegal access flag for GTZC1 TZSC registers"]
    #[inline(always)]
    #[must_use]
    pub fn ctzsc1f(&mut self) -> CTZSC1F_W<FCR4rs> {
        CTZSC1F_W::new(self, 14)
    }
    #[doc = "Bit 15 - clear the illegal access flag for GTZC1 TZIC registers"]
    #[inline(always)]
    #[must_use]
    pub fn ctzic1f(&mut self) -> CTZIC1F_W<FCR4rs> {
        CTZIC1F_W::new(self, 15)
    }
    #[doc = "Bit 16 - clear the illegal access flag for MPCWM1 (OCTOSPI1) memory bank"]
    #[inline(always)]
    #[must_use]
    pub fn coctospi1_memf(&mut self) -> COCTOSPI1_MEMF_W<FCR4rs> {
        COCTOSPI1_MEMF_W::new(self, 16)
    }
    #[doc = "Bit 18 - clear the illegal access flag for MPCWM3 (BKPSRAM) memory bank"]
    #[inline(always)]
    #[must_use]
    pub fn cbkpsramf(&mut self) -> CBKPSRAMF_W<FCR4rs> {
        CBKPSRAMF_W::new(self, 18)
    }
    #[doc = "Bit 20 - clear the illegal access flag for HSPI1 memory bank"]
    #[inline(always)]
    #[must_use]
    pub fn chspi1_memf(&mut self) -> CHSPI1_MEMF_W<FCR4rs> {
        CHSPI1_MEMF_W::new(self, 20)
    }
    #[doc = "Bit 24 - clear the illegal access flag for SRAM1"]
    #[inline(always)]
    #[must_use]
    pub fn csram1f(&mut self) -> CSRAM1F_W<FCR4rs> {
        CSRAM1F_W::new(self, 24)
    }
    #[doc = "Bit 25 - clear the illegal access flag for MPCBB1 registers"]
    #[inline(always)]
    #[must_use]
    pub fn cmpcbb1_regf(&mut self) -> CMPCBB1_REGF_W<FCR4rs> {
        CMPCBB1_REGF_W::new(self, 25)
    }
    #[doc = "Bit 26 - clear the illegal access flag for SRAM2"]
    #[inline(always)]
    #[must_use]
    pub fn csram2f(&mut self) -> CSRAM2F_W<FCR4rs> {
        CSRAM2F_W::new(self, 26)
    }
    #[doc = "Bit 27 - clear the illegal access flag for MPCBB2 registers"]
    #[inline(always)]
    #[must_use]
    pub fn cmpcbb2_regf(&mut self) -> CMPCBB2_REGF_W<FCR4rs> {
        CMPCBB2_REGF_W::new(self, 27)
    }
    #[doc = "Bit 30 - clear the illegal access flag for SRAM5"]
    #[inline(always)]
    #[must_use]
    pub fn csram5f(&mut self) -> CSRAM5F_W<FCR4rs> {
        CSRAM5F_W::new(self, 30)
    }
}
#[doc = "TZIC flag clear register 4\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcr4::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FCR4rs;
impl crate::RegisterSpec for FCR4rs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`fcr4::W`](W) writer structure"]
impl crate::Writable for FCR4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FCR4 to value 0"]
impl crate::Resettable for FCR4rs {
    const RESET_VALUE: u32 = 0;
}
