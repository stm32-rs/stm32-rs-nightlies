#[doc = "Register `HYSCR3` reader"]
pub type R = crate::R<HYSCR3rs>;
#[doc = "Register `HYSCR3` writer"]
pub type W = crate::W<HYSCR3rs>;
#[doc = "Field `PE` reader - Port E hysteresis control on/off"]
pub type PE_R = crate::FieldReader<u16>;
#[doc = "Field `PE` writer - Port E hysteresis control on/off"]
pub type PE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PF` reader - Port F hysteresis control on/off"]
pub type PF_R = crate::FieldReader<u16>;
#[doc = "Field `PF` writer - Port F hysteresis control on/off"]
pub type PF_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Port E hysteresis control on/off"]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Port F hysteresis control on/off"]
    #[inline(always)]
    pub fn pf(&self) -> PF_R {
        PF_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Port E hysteresis control on/off"]
    #[inline(always)]
    #[must_use]
    pub fn pe(&mut self) -> PE_W<HYSCR3rs> {
        PE_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Port F hysteresis control on/off"]
    #[inline(always)]
    #[must_use]
    pub fn pf(&mut self) -> PF_W<HYSCR3rs> {
        PF_W::new(self, 16)
    }
}
#[doc = "RI hysteresis control register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hyscr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hyscr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HYSCR3rs;
impl crate::RegisterSpec for HYSCR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hyscr3::R`](R) reader structure"]
impl crate::Readable for HYSCR3rs {}
#[doc = "`write(|w| ..)` method takes [`hyscr3::W`](W) writer structure"]
impl crate::Writable for HYSCR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HYSCR3 to value 0"]
impl crate::Resettable for HYSCR3rs {
    const RESET_VALUE: u32 = 0;
}
