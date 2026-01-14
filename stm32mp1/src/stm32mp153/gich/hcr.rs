///Register `HCR` reader
pub type R = crate::R<HCRrs>;
///Register `HCR` writer
pub type W = crate::W<HCRrs>;
///Field `EN` reader - EN
pub type EN_R = crate::BitReader;
///Field `EN` writer - EN
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UIE` reader - UIE
pub type UIE_R = crate::BitReader;
///Field `UIE` writer - UIE
pub type UIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LRENPIE` reader - LRENPIE
pub type LRENPIE_R = crate::BitReader;
///Field `LRENPIE` writer - LRENPIE
pub type LRENPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NPIE` reader - NPIE
pub type NPIE_R = crate::BitReader;
///Field `NPIE` writer - NPIE
pub type NPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VGRP0EIE` reader - VGRP0EIE
pub type VGRP0EIE_R = crate::BitReader;
///Field `VGRP0EIE` writer - VGRP0EIE
pub type VGRP0EIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VGRP0DIE` reader - VGRP0DIE
pub type VGRP0DIE_R = crate::BitReader;
///Field `VGRP0DIE` writer - VGRP0DIE
pub type VGRP0DIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VGRP1EIE` reader - VGRP1EIE
pub type VGRP1EIE_R = crate::BitReader;
///Field `VGRP1EIE` writer - VGRP1EIE
pub type VGRP1EIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VGRP1DIE` reader - VGRP1DIE
pub type VGRP1DIE_R = crate::BitReader;
///Field `VGRP1DIE` writer - VGRP1DIE
pub type VGRP1DIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOICOUNT` reader - EOICOUNT
pub type EOICOUNT_R = crate::FieldReader;
///Field `EOICOUNT` writer - EOICOUNT
pub type EOICOUNT_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bit 0 - EN
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - UIE
    #[inline(always)]
    pub fn uie(&self) -> UIE_R {
        UIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - LRENPIE
    #[inline(always)]
    pub fn lrenpie(&self) -> LRENPIE_R {
        LRENPIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - NPIE
    #[inline(always)]
    pub fn npie(&self) -> NPIE_R {
        NPIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - VGRP0EIE
    #[inline(always)]
    pub fn vgrp0eie(&self) -> VGRP0EIE_R {
        VGRP0EIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - VGRP0DIE
    #[inline(always)]
    pub fn vgrp0die(&self) -> VGRP0DIE_R {
        VGRP0DIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - VGRP1EIE
    #[inline(always)]
    pub fn vgrp1eie(&self) -> VGRP1EIE_R {
        VGRP1EIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - VGRP1DIE
    #[inline(always)]
    pub fn vgrp1die(&self) -> VGRP1DIE_R {
        VGRP1DIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 27:31 - EOICOUNT
    #[inline(always)]
    pub fn eoicount(&self) -> EOICOUNT_R {
        EOICOUNT_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCR")
            .field("en", &self.en())
            .field("uie", &self.uie())
            .field("lrenpie", &self.lrenpie())
            .field("npie", &self.npie())
            .field("vgrp0eie", &self.vgrp0eie())
            .field("vgrp0die", &self.vgrp0die())
            .field("vgrp1eie", &self.vgrp1eie())
            .field("vgrp1die", &self.vgrp1die())
            .field("eoicount", &self.eoicount())
            .finish()
    }
}
impl W {
    ///Bit 0 - EN
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<'_, HCRrs> {
        EN_W::new(self, 0)
    }
    ///Bit 1 - UIE
    #[inline(always)]
    pub fn uie(&mut self) -> UIE_W<'_, HCRrs> {
        UIE_W::new(self, 1)
    }
    ///Bit 2 - LRENPIE
    #[inline(always)]
    pub fn lrenpie(&mut self) -> LRENPIE_W<'_, HCRrs> {
        LRENPIE_W::new(self, 2)
    }
    ///Bit 3 - NPIE
    #[inline(always)]
    pub fn npie(&mut self) -> NPIE_W<'_, HCRrs> {
        NPIE_W::new(self, 3)
    }
    ///Bit 4 - VGRP0EIE
    #[inline(always)]
    pub fn vgrp0eie(&mut self) -> VGRP0EIE_W<'_, HCRrs> {
        VGRP0EIE_W::new(self, 4)
    }
    ///Bit 5 - VGRP0DIE
    #[inline(always)]
    pub fn vgrp0die(&mut self) -> VGRP0DIE_W<'_, HCRrs> {
        VGRP0DIE_W::new(self, 5)
    }
    ///Bit 6 - VGRP1EIE
    #[inline(always)]
    pub fn vgrp1eie(&mut self) -> VGRP1EIE_W<'_, HCRrs> {
        VGRP1EIE_W::new(self, 6)
    }
    ///Bit 7 - VGRP1DIE
    #[inline(always)]
    pub fn vgrp1die(&mut self) -> VGRP1DIE_W<'_, HCRrs> {
        VGRP1DIE_W::new(self, 7)
    }
    ///Bits 27:31 - EOICOUNT
    #[inline(always)]
    pub fn eoicount(&mut self) -> EOICOUNT_W<'_, HCRrs> {
        EOICOUNT_W::new(self, 27)
    }
}
/**GICH hypervisor control register

You can [`read`](crate::Reg::read) this register and get [`hcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICH:HCR)*/
pub struct HCRrs;
impl crate::RegisterSpec for HCRrs {
    type Ux = u32;
}
///`read()` method returns [`hcr::R`](R) reader structure
impl crate::Readable for HCRrs {}
///`write(|w| ..)` method takes [`hcr::W`](W) writer structure
impl crate::Writable for HCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HCR to value 0
impl crate::Resettable for HCRrs {}
