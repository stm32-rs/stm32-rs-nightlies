#[doc = "Register `ISR` reader"]
pub type R = crate::R<ISRrs>;
#[doc = "Register `ISR` writer"]
pub type W = crate::W<ISRrs>;
#[doc = "Field `ADRDY` reader - ADRDY"]
pub type ADRDY_R = crate::BitReader;
#[doc = "Field `ADRDY` writer - ADRDY"]
pub type ADRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOSMP` reader - EOSMP"]
pub type EOSMP_R = crate::BitReader;
#[doc = "Field `EOSMP` writer - EOSMP"]
pub type EOSMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOC` reader - EOC"]
pub type EOC_R = crate::BitReader;
#[doc = "Field `EOC` writer - EOC"]
pub type EOC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOS` reader - EOS"]
pub type EOS_R = crate::BitReader;
#[doc = "Field `EOS` writer - EOS"]
pub type EOS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVR` reader - OVR"]
pub type OVR_R = crate::BitReader;
#[doc = "Field `OVR` writer - OVR"]
pub type OVR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWD1` reader - AWD1"]
pub type AWD1_R = crate::BitReader;
#[doc = "Field `AWD1` writer - AWD1"]
pub type AWD1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWD2` reader - AWD2"]
pub type AWD2_R = crate::BitReader;
#[doc = "Field `AWD2` writer - AWD2"]
pub type AWD2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWD3` reader - AWD3"]
pub type AWD3_R = crate::BitReader;
#[doc = "Field `AWD3` writer - AWD3"]
pub type AWD3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOCAL` reader - EOCAL"]
pub type EOCAL_R = crate::BitReader;
#[doc = "Field `EOCAL` writer - EOCAL"]
pub type EOCAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCRDY` reader - CCRDY"]
pub type CCRDY_R = crate::BitReader;
#[doc = "Field `CCRDY` writer - CCRDY"]
pub type CCRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ADRDY"]
    #[inline(always)]
    pub fn adrdy(&self) -> ADRDY_R {
        ADRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EOSMP"]
    #[inline(always)]
    pub fn eosmp(&self) -> EOSMP_R {
        EOSMP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EOC"]
    #[inline(always)]
    pub fn eoc(&self) -> EOC_R {
        EOC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EOS"]
    #[inline(always)]
    pub fn eos(&self) -> EOS_R {
        EOS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OVR"]
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - AWD1"]
    #[inline(always)]
    pub fn awd1(&self) -> AWD1_R {
        AWD1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - AWD2"]
    #[inline(always)]
    pub fn awd2(&self) -> AWD2_R {
        AWD2_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AWD3"]
    #[inline(always)]
    pub fn awd3(&self) -> AWD3_R {
        AWD3_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - EOCAL"]
    #[inline(always)]
    pub fn eocal(&self) -> EOCAL_R {
        EOCAL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - CCRDY"]
    #[inline(always)]
    pub fn ccrdy(&self) -> CCRDY_R {
        CCRDY_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADRDY"]
    #[inline(always)]
    #[must_use]
    pub fn adrdy(&mut self) -> ADRDY_W<ISRrs> {
        ADRDY_W::new(self, 0)
    }
    #[doc = "Bit 1 - EOSMP"]
    #[inline(always)]
    #[must_use]
    pub fn eosmp(&mut self) -> EOSMP_W<ISRrs> {
        EOSMP_W::new(self, 1)
    }
    #[doc = "Bit 2 - EOC"]
    #[inline(always)]
    #[must_use]
    pub fn eoc(&mut self) -> EOC_W<ISRrs> {
        EOC_W::new(self, 2)
    }
    #[doc = "Bit 3 - EOS"]
    #[inline(always)]
    #[must_use]
    pub fn eos(&mut self) -> EOS_W<ISRrs> {
        EOS_W::new(self, 3)
    }
    #[doc = "Bit 4 - OVR"]
    #[inline(always)]
    #[must_use]
    pub fn ovr(&mut self) -> OVR_W<ISRrs> {
        OVR_W::new(self, 4)
    }
    #[doc = "Bit 7 - AWD1"]
    #[inline(always)]
    #[must_use]
    pub fn awd1(&mut self) -> AWD1_W<ISRrs> {
        AWD1_W::new(self, 7)
    }
    #[doc = "Bit 8 - AWD2"]
    #[inline(always)]
    #[must_use]
    pub fn awd2(&mut self) -> AWD2_W<ISRrs> {
        AWD2_W::new(self, 8)
    }
    #[doc = "Bit 9 - AWD3"]
    #[inline(always)]
    #[must_use]
    pub fn awd3(&mut self) -> AWD3_W<ISRrs> {
        AWD3_W::new(self, 9)
    }
    #[doc = "Bit 11 - EOCAL"]
    #[inline(always)]
    #[must_use]
    pub fn eocal(&mut self) -> EOCAL_W<ISRrs> {
        EOCAL_W::new(self, 11)
    }
    #[doc = "Bit 13 - CCRDY"]
    #[inline(always)]
    #[must_use]
    pub fn ccrdy(&mut self) -> CCRDY_W<ISRrs> {
        CCRDY_W::new(self, 13)
    }
}
#[doc = "ADC interrupt and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for ISRrs {}
#[doc = "`write(|w| ..)` method takes [`isr::W`](W) writer structure"]
impl crate::Writable for ISRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for ISRrs {
    const RESET_VALUE: u32 = 0;
}
