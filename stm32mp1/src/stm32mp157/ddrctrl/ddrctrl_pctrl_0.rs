#[doc = "Register `DDRCTRL_PCTRL_0` reader"]
pub type R = crate::R<DDRCTRL_PCTRL_0rs>;
#[doc = "Register `DDRCTRL_PCTRL_0` writer"]
pub type W = crate::W<DDRCTRL_PCTRL_0rs>;
#[doc = "Field `PORT_EN` reader - PORT_EN"]
pub type PORT_EN_R = crate::BitReader;
#[doc = "Field `PORT_EN` writer - PORT_EN"]
pub type PORT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PORT_EN"]
    #[inline(always)]
    pub fn port_en(&self) -> PORT_EN_R {
        PORT_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PORT_EN"]
    #[inline(always)]
    #[must_use]
    pub fn port_en(&mut self) -> PORT_EN_W<DDRCTRL_PCTRL_0rs> {
        PORT_EN_W::new(self, 0)
    }
}
#[doc = "DDRCTRL port 0 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_pctrl_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_pctrl_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRCTRL_PCTRL_0rs;
impl crate::RegisterSpec for DDRCTRL_PCTRL_0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrctrl_pctrl_0::R`](R) reader structure"]
impl crate::Readable for DDRCTRL_PCTRL_0rs {}
#[doc = "`write(|w| ..)` method takes [`ddrctrl_pctrl_0::W`](W) writer structure"]
impl crate::Writable for DDRCTRL_PCTRL_0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRCTRL_PCTRL_0 to value 0"]
impl crate::Resettable for DDRCTRL_PCTRL_0rs {
    const RESET_VALUE: u32 = 0;
}
