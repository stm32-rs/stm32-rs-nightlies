#[doc = "Register `HYSCR2` reader"]
pub type R = crate::R<HYSCR2rs>;
#[doc = "Register `HYSCR2` writer"]
pub type W = crate::W<HYSCR2rs>;
#[doc = "Field `PC` reader - Port C hysteresis control on/off"]
pub type PC_R = crate::FieldReader<u16>;
#[doc = "Field `PC` writer - Port C hysteresis control on/off"]
pub type PC_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PD` reader - Port D hysteresis control on/off"]
pub type PD_R = crate::FieldReader<u16>;
#[doc = "Field `PD` writer - Port D hysteresis control on/off"]
pub type PD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Port C hysteresis control on/off"]
    #[inline(always)]
    pub fn pc(&self) -> PC_R {
        PC_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Port D hysteresis control on/off"]
    #[inline(always)]
    pub fn pd(&self) -> PD_R {
        PD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Port C hysteresis control on/off"]
    #[inline(always)]
    #[must_use]
    pub fn pc(&mut self) -> PC_W<HYSCR2rs> {
        PC_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Port D hysteresis control on/off"]
    #[inline(always)]
    #[must_use]
    pub fn pd(&mut self) -> PD_W<HYSCR2rs> {
        PD_W::new(self, 16)
    }
}
#[doc = "RI hysteresis control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hyscr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hyscr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HYSCR2rs;
impl crate::RegisterSpec for HYSCR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hyscr2::R`](R) reader structure"]
impl crate::Readable for HYSCR2rs {}
#[doc = "`write(|w| ..)` method takes [`hyscr2::W`](W) writer structure"]
impl crate::Writable for HYSCR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HYSCR2 to value 0"]
impl crate::Resettable for HYSCR2rs {
    const RESET_VALUE: u32 = 0;
}
