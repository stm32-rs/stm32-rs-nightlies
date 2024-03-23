#[doc = "Register `FDCAN_IE` reader"]
pub type R = crate::R<FDCAN_IErs>;
#[doc = "Register `FDCAN_IE` writer"]
pub type W = crate::W<FDCAN_IErs>;
#[doc = "Field `RF0NE` reader - RF0NE"]
pub type RF0NE_R = crate::BitReader;
#[doc = "Field `RF0NE` writer - RF0NE"]
pub type RF0NE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF0WE` reader - RF0WE"]
pub type RF0WE_R = crate::BitReader;
#[doc = "Field `RF0WE` writer - RF0WE"]
pub type RF0WE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF0FE` reader - RF0FE"]
pub type RF0FE_R = crate::BitReader;
#[doc = "Field `RF0FE` writer - RF0FE"]
pub type RF0FE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF0LE` reader - RF0LE"]
pub type RF0LE_R = crate::BitReader;
#[doc = "Field `RF0LE` writer - RF0LE"]
pub type RF0LE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF1NE` reader - RF1NE"]
pub type RF1NE_R = crate::BitReader;
#[doc = "Field `RF1NE` writer - RF1NE"]
pub type RF1NE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF1WE` reader - RF1WE"]
pub type RF1WE_R = crate::BitReader;
#[doc = "Field `RF1WE` writer - RF1WE"]
pub type RF1WE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF1FE` reader - RF1FE"]
pub type RF1FE_R = crate::BitReader;
#[doc = "Field `RF1FE` writer - RF1FE"]
pub type RF1FE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF1LE` reader - RF1LE"]
pub type RF1LE_R = crate::BitReader;
#[doc = "Field `RF1LE` writer - RF1LE"]
pub type RF1LE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPME` reader - HPME"]
pub type HPME_R = crate::BitReader;
#[doc = "Field `HPME` writer - HPME"]
pub type HPME_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCE` reader - TCE"]
pub type TCE_R = crate::BitReader;
#[doc = "Field `TCE` writer - TCE"]
pub type TCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCFE` reader - TCFE"]
pub type TCFE_R = crate::BitReader;
#[doc = "Field `TCFE` writer - TCFE"]
pub type TCFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFEE` reader - TFEE"]
pub type TFEE_R = crate::BitReader;
#[doc = "Field `TFEE` writer - TFEE"]
pub type TFEE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEFNE` reader - TEFNE"]
pub type TEFNE_R = crate::BitReader;
#[doc = "Field `TEFNE` writer - TEFNE"]
pub type TEFNE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEFWE` reader - TEFWE"]
pub type TEFWE_R = crate::BitReader;
#[doc = "Field `TEFWE` writer - TEFWE"]
pub type TEFWE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEFFE` reader - TEFFE"]
pub type TEFFE_R = crate::BitReader;
#[doc = "Field `TEFFE` writer - TEFFE"]
pub type TEFFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEFLE` reader - TEFLE"]
pub type TEFLE_R = crate::BitReader;
#[doc = "Field `TEFLE` writer - TEFLE"]
pub type TEFLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSWE` reader - TSWE"]
pub type TSWE_R = crate::BitReader;
#[doc = "Field `TSWE` writer - TSWE"]
pub type TSWE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MRAFE` reader - MRAFE"]
pub type MRAFE_R = crate::BitReader;
#[doc = "Field `MRAFE` writer - MRAFE"]
pub type MRAFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOOE` reader - TOOE"]
pub type TOOE_R = crate::BitReader;
#[doc = "Field `TOOE` writer - TOOE"]
pub type TOOE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRXE` reader - DRXE"]
pub type DRXE_R = crate::BitReader;
#[doc = "Field `DRXE` writer - DRXE"]
pub type DRXE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BECE` reader - BECE"]
pub type BECE_R = crate::BitReader;
#[doc = "Field `BECE` writer - BECE"]
pub type BECE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BEUE` reader - BEUE"]
pub type BEUE_R = crate::BitReader;
#[doc = "Field `BEUE` writer - BEUE"]
pub type BEUE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ELOE` reader - ELOE"]
pub type ELOE_R = crate::BitReader;
#[doc = "Field `ELOE` writer - ELOE"]
pub type ELOE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPE` reader - EPE"]
pub type EPE_R = crate::BitReader;
#[doc = "Field `EPE` writer - EPE"]
pub type EPE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EWE` reader - EWE"]
pub type EWE_R = crate::BitReader;
#[doc = "Field `EWE` writer - EWE"]
pub type EWE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOE` reader - BOE"]
pub type BOE_R = crate::BitReader;
#[doc = "Field `BOE` writer - BOE"]
pub type BOE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDIE` reader - WDIE"]
pub type WDIE_R = crate::BitReader;
#[doc = "Field `WDIE` writer - WDIE"]
pub type WDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEAE` reader - PEAE"]
pub type PEAE_R = crate::BitReader;
#[doc = "Field `PEAE` writer - PEAE"]
pub type PEAE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEDE` reader - PEDE"]
pub type PEDE_R = crate::BitReader;
#[doc = "Field `PEDE` writer - PEDE"]
pub type PEDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARAE` reader - ARAE"]
pub type ARAE_R = crate::BitReader;
#[doc = "Field `ARAE` writer - ARAE"]
pub type ARAE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RF0NE"]
    #[inline(always)]
    pub fn rf0ne(&self) -> RF0NE_R {
        RF0NE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RF0WE"]
    #[inline(always)]
    pub fn rf0we(&self) -> RF0WE_R {
        RF0WE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RF0FE"]
    #[inline(always)]
    pub fn rf0fe(&self) -> RF0FE_R {
        RF0FE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RF0LE"]
    #[inline(always)]
    pub fn rf0le(&self) -> RF0LE_R {
        RF0LE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RF1NE"]
    #[inline(always)]
    pub fn rf1ne(&self) -> RF1NE_R {
        RF1NE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RF1WE"]
    #[inline(always)]
    pub fn rf1we(&self) -> RF1WE_R {
        RF1WE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RF1FE"]
    #[inline(always)]
    pub fn rf1fe(&self) -> RF1FE_R {
        RF1FE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RF1LE"]
    #[inline(always)]
    pub fn rf1le(&self) -> RF1LE_R {
        RF1LE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - HPME"]
    #[inline(always)]
    pub fn hpme(&self) -> HPME_R {
        HPME_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TCE"]
    #[inline(always)]
    pub fn tce(&self) -> TCE_R {
        TCE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TCFE"]
    #[inline(always)]
    pub fn tcfe(&self) -> TCFE_R {
        TCFE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TFEE"]
    #[inline(always)]
    pub fn tfee(&self) -> TFEE_R {
        TFEE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TEFNE"]
    #[inline(always)]
    pub fn tefne(&self) -> TEFNE_R {
        TEFNE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TEFWE"]
    #[inline(always)]
    pub fn tefwe(&self) -> TEFWE_R {
        TEFWE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TEFFE"]
    #[inline(always)]
    pub fn teffe(&self) -> TEFFE_R {
        TEFFE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TEFLE"]
    #[inline(always)]
    pub fn tefle(&self) -> TEFLE_R {
        TEFLE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - TSWE"]
    #[inline(always)]
    pub fn tswe(&self) -> TSWE_R {
        TSWE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - MRAFE"]
    #[inline(always)]
    pub fn mrafe(&self) -> MRAFE_R {
        MRAFE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TOOE"]
    #[inline(always)]
    pub fn tooe(&self) -> TOOE_R {
        TOOE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DRXE"]
    #[inline(always)]
    pub fn drxe(&self) -> DRXE_R {
        DRXE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - BECE"]
    #[inline(always)]
    pub fn bece(&self) -> BECE_R {
        BECE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - BEUE"]
    #[inline(always)]
    pub fn beue(&self) -> BEUE_R {
        BEUE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - ELOE"]
    #[inline(always)]
    pub fn eloe(&self) -> ELOE_R {
        ELOE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - EPE"]
    #[inline(always)]
    pub fn epe(&self) -> EPE_R {
        EPE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - EWE"]
    #[inline(always)]
    pub fn ewe(&self) -> EWE_R {
        EWE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - BOE"]
    #[inline(always)]
    pub fn boe(&self) -> BOE_R {
        BOE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - WDIE"]
    #[inline(always)]
    pub fn wdie(&self) -> WDIE_R {
        WDIE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - PEAE"]
    #[inline(always)]
    pub fn peae(&self) -> PEAE_R {
        PEAE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - PEDE"]
    #[inline(always)]
    pub fn pede(&self) -> PEDE_R {
        PEDE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - ARAE"]
    #[inline(always)]
    pub fn arae(&self) -> ARAE_R {
        ARAE_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RF0NE"]
    #[inline(always)]
    #[must_use]
    pub fn rf0ne(&mut self) -> RF0NE_W<FDCAN_IErs> {
        RF0NE_W::new(self, 0)
    }
    #[doc = "Bit 1 - RF0WE"]
    #[inline(always)]
    #[must_use]
    pub fn rf0we(&mut self) -> RF0WE_W<FDCAN_IErs> {
        RF0WE_W::new(self, 1)
    }
    #[doc = "Bit 2 - RF0FE"]
    #[inline(always)]
    #[must_use]
    pub fn rf0fe(&mut self) -> RF0FE_W<FDCAN_IErs> {
        RF0FE_W::new(self, 2)
    }
    #[doc = "Bit 3 - RF0LE"]
    #[inline(always)]
    #[must_use]
    pub fn rf0le(&mut self) -> RF0LE_W<FDCAN_IErs> {
        RF0LE_W::new(self, 3)
    }
    #[doc = "Bit 4 - RF1NE"]
    #[inline(always)]
    #[must_use]
    pub fn rf1ne(&mut self) -> RF1NE_W<FDCAN_IErs> {
        RF1NE_W::new(self, 4)
    }
    #[doc = "Bit 5 - RF1WE"]
    #[inline(always)]
    #[must_use]
    pub fn rf1we(&mut self) -> RF1WE_W<FDCAN_IErs> {
        RF1WE_W::new(self, 5)
    }
    #[doc = "Bit 6 - RF1FE"]
    #[inline(always)]
    #[must_use]
    pub fn rf1fe(&mut self) -> RF1FE_W<FDCAN_IErs> {
        RF1FE_W::new(self, 6)
    }
    #[doc = "Bit 7 - RF1LE"]
    #[inline(always)]
    #[must_use]
    pub fn rf1le(&mut self) -> RF1LE_W<FDCAN_IErs> {
        RF1LE_W::new(self, 7)
    }
    #[doc = "Bit 8 - HPME"]
    #[inline(always)]
    #[must_use]
    pub fn hpme(&mut self) -> HPME_W<FDCAN_IErs> {
        HPME_W::new(self, 8)
    }
    #[doc = "Bit 9 - TCE"]
    #[inline(always)]
    #[must_use]
    pub fn tce(&mut self) -> TCE_W<FDCAN_IErs> {
        TCE_W::new(self, 9)
    }
    #[doc = "Bit 10 - TCFE"]
    #[inline(always)]
    #[must_use]
    pub fn tcfe(&mut self) -> TCFE_W<FDCAN_IErs> {
        TCFE_W::new(self, 10)
    }
    #[doc = "Bit 11 - TFEE"]
    #[inline(always)]
    #[must_use]
    pub fn tfee(&mut self) -> TFEE_W<FDCAN_IErs> {
        TFEE_W::new(self, 11)
    }
    #[doc = "Bit 12 - TEFNE"]
    #[inline(always)]
    #[must_use]
    pub fn tefne(&mut self) -> TEFNE_W<FDCAN_IErs> {
        TEFNE_W::new(self, 12)
    }
    #[doc = "Bit 13 - TEFWE"]
    #[inline(always)]
    #[must_use]
    pub fn tefwe(&mut self) -> TEFWE_W<FDCAN_IErs> {
        TEFWE_W::new(self, 13)
    }
    #[doc = "Bit 14 - TEFFE"]
    #[inline(always)]
    #[must_use]
    pub fn teffe(&mut self) -> TEFFE_W<FDCAN_IErs> {
        TEFFE_W::new(self, 14)
    }
    #[doc = "Bit 15 - TEFLE"]
    #[inline(always)]
    #[must_use]
    pub fn tefle(&mut self) -> TEFLE_W<FDCAN_IErs> {
        TEFLE_W::new(self, 15)
    }
    #[doc = "Bit 16 - TSWE"]
    #[inline(always)]
    #[must_use]
    pub fn tswe(&mut self) -> TSWE_W<FDCAN_IErs> {
        TSWE_W::new(self, 16)
    }
    #[doc = "Bit 17 - MRAFE"]
    #[inline(always)]
    #[must_use]
    pub fn mrafe(&mut self) -> MRAFE_W<FDCAN_IErs> {
        MRAFE_W::new(self, 17)
    }
    #[doc = "Bit 18 - TOOE"]
    #[inline(always)]
    #[must_use]
    pub fn tooe(&mut self) -> TOOE_W<FDCAN_IErs> {
        TOOE_W::new(self, 18)
    }
    #[doc = "Bit 19 - DRXE"]
    #[inline(always)]
    #[must_use]
    pub fn drxe(&mut self) -> DRXE_W<FDCAN_IErs> {
        DRXE_W::new(self, 19)
    }
    #[doc = "Bit 20 - BECE"]
    #[inline(always)]
    #[must_use]
    pub fn bece(&mut self) -> BECE_W<FDCAN_IErs> {
        BECE_W::new(self, 20)
    }
    #[doc = "Bit 21 - BEUE"]
    #[inline(always)]
    #[must_use]
    pub fn beue(&mut self) -> BEUE_W<FDCAN_IErs> {
        BEUE_W::new(self, 21)
    }
    #[doc = "Bit 22 - ELOE"]
    #[inline(always)]
    #[must_use]
    pub fn eloe(&mut self) -> ELOE_W<FDCAN_IErs> {
        ELOE_W::new(self, 22)
    }
    #[doc = "Bit 23 - EPE"]
    #[inline(always)]
    #[must_use]
    pub fn epe(&mut self) -> EPE_W<FDCAN_IErs> {
        EPE_W::new(self, 23)
    }
    #[doc = "Bit 24 - EWE"]
    #[inline(always)]
    #[must_use]
    pub fn ewe(&mut self) -> EWE_W<FDCAN_IErs> {
        EWE_W::new(self, 24)
    }
    #[doc = "Bit 25 - BOE"]
    #[inline(always)]
    #[must_use]
    pub fn boe(&mut self) -> BOE_W<FDCAN_IErs> {
        BOE_W::new(self, 25)
    }
    #[doc = "Bit 26 - WDIE"]
    #[inline(always)]
    #[must_use]
    pub fn wdie(&mut self) -> WDIE_W<FDCAN_IErs> {
        WDIE_W::new(self, 26)
    }
    #[doc = "Bit 27 - PEAE"]
    #[inline(always)]
    #[must_use]
    pub fn peae(&mut self) -> PEAE_W<FDCAN_IErs> {
        PEAE_W::new(self, 27)
    }
    #[doc = "Bit 28 - PEDE"]
    #[inline(always)]
    #[must_use]
    pub fn pede(&mut self) -> PEDE_W<FDCAN_IErs> {
        PEDE_W::new(self, 28)
    }
    #[doc = "Bit 29 - ARAE"]
    #[inline(always)]
    #[must_use]
    pub fn arae(&mut self) -> ARAE_W<FDCAN_IErs> {
        ARAE_W::new(self, 29)
    }
}
#[doc = "The settings in the interrupt enable register determine which status changes in the interrupt register will be signaled on an interrupt line.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ie::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_ie::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_IErs;
impl crate::RegisterSpec for FDCAN_IErs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_ie::R`](R) reader structure"]
impl crate::Readable for FDCAN_IErs {}
#[doc = "`write(|w| ..)` method takes [`fdcan_ie::W`](W) writer structure"]
impl crate::Writable for FDCAN_IErs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FDCAN_IE to value 0"]
impl crate::Resettable for FDCAN_IErs {
    const RESET_VALUE: u32 = 0;
}
