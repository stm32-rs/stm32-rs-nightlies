#[doc = "Register `HYSCR4` reader"]
pub type R = crate::R<HYSCR4rs>;
#[doc = "Register `HYSCR4` writer"]
pub type W = crate::W<HYSCR4rs>;
#[doc = "Field `PG` reader - Port G hysteresis control on/off"]
pub type PG_R = crate::FieldReader<u16>;
#[doc = "Field `PG` writer - Port G hysteresis control on/off"]
pub type PG_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Port G hysteresis control on/off"]
    #[inline(always)]
    pub fn pg(&self) -> PG_R {
        PG_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Port G hysteresis control on/off"]
    #[inline(always)]
    #[must_use]
    pub fn pg(&mut self) -> PG_W<HYSCR4rs> {
        PG_W::new(self, 0)
    }
}
#[doc = "Hysteresis control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hyscr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hyscr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HYSCR4rs;
impl crate::RegisterSpec for HYSCR4rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hyscr4::R`](R) reader structure"]
impl crate::Readable for HYSCR4rs {}
#[doc = "`write(|w| ..)` method takes [`hyscr4::W`](W) writer structure"]
impl crate::Writable for HYSCR4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HYSCR4 to value 0"]
impl crate::Resettable for HYSCR4rs {
    const RESET_VALUE: u32 = 0;
}
