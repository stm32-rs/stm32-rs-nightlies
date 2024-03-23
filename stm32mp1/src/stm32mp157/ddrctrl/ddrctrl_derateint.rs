#[doc = "Register `DDRCTRL_DERATEINT` reader"]
pub type R = crate::R<DDRCTRL_DERATEINTrs>;
#[doc = "Register `DDRCTRL_DERATEINT` writer"]
pub type W = crate::W<DDRCTRL_DERATEINTrs>;
#[doc = "Field `MR4_READ_INTERVAL` reader - MR4_READ_INTERVAL"]
pub type MR4_READ_INTERVAL_R = crate::FieldReader<u32>;
#[doc = "Field `MR4_READ_INTERVAL` writer - MR4_READ_INTERVAL"]
pub type MR4_READ_INTERVAL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MR4_READ_INTERVAL"]
    #[inline(always)]
    pub fn mr4_read_interval(&self) -> MR4_READ_INTERVAL_R {
        MR4_READ_INTERVAL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MR4_READ_INTERVAL"]
    #[inline(always)]
    #[must_use]
    pub fn mr4_read_interval(&mut self) -> MR4_READ_INTERVAL_W<DDRCTRL_DERATEINTrs> {
        MR4_READ_INTERVAL_W::new(self, 0)
    }
}
#[doc = "DDRCTRL temperature derate interval register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_derateint::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_derateint::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRCTRL_DERATEINTrs;
impl crate::RegisterSpec for DDRCTRL_DERATEINTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrctrl_derateint::R`](R) reader structure"]
impl crate::Readable for DDRCTRL_DERATEINTrs {}
#[doc = "`write(|w| ..)` method takes [`ddrctrl_derateint::W`](W) writer structure"]
impl crate::Writable for DDRCTRL_DERATEINTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRCTRL_DERATEINT to value 0x0080_0000"]
impl crate::Resettable for DDRCTRL_DERATEINTrs {
    const RESET_VALUE: u32 = 0x0080_0000;
}
