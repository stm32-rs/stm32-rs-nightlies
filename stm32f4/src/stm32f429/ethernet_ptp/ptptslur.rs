#[doc = "Register `PTPTSLUR` reader"]
pub type R = crate::R<PTPTSLURrs>;
#[doc = "Register `PTPTSLUR` writer"]
pub type W = crate::W<PTPTSLURrs>;
#[doc = "Field `TSUSS` reader - TSUSS"]
pub type TSUSS_R = crate::FieldReader<u32>;
#[doc = "Field `TSUSS` writer - TSUSS"]
pub type TSUSS_W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
#[doc = "Field `TSUPNS` reader - TSUSS"]
pub type TSUPNS_R = crate::BitReader;
#[doc = "Field `TSUPNS` writer - TSUSS"]
pub type TSUPNS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:30 - TSUSS"]
    #[inline(always)]
    pub fn tsuss(&self) -> TSUSS_R {
        TSUSS_R::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - TSUSS"]
    #[inline(always)]
    pub fn tsupns(&self) -> TSUPNS_R {
        TSUPNS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - TSUSS"]
    #[inline(always)]
    #[must_use]
    pub fn tsuss(&mut self) -> TSUSS_W<PTPTSLURrs> {
        TSUSS_W::new(self, 0)
    }
    #[doc = "Bit 31 - TSUSS"]
    #[inline(always)]
    #[must_use]
    pub fn tsupns(&mut self) -> TSUPNS_W<PTPTSLURrs> {
        TSUPNS_W::new(self, 31)
    }
}
#[doc = "Ethernet PTP time stamp low update register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptptslur::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptptslur::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PTPTSLURrs;
impl crate::RegisterSpec for PTPTSLURrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptptslur::R`](R) reader structure"]
impl crate::Readable for PTPTSLURrs {}
#[doc = "`write(|w| ..)` method takes [`ptptslur::W`](W) writer structure"]
impl crate::Writable for PTPTSLURrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PTPTSLUR to value 0"]
impl crate::Resettable for PTPTSLURrs {
    const RESET_VALUE: u32 = 0;
}
