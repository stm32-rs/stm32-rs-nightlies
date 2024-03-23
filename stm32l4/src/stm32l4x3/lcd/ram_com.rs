#[doc = "Register `RAM_COM%s` reader"]
pub type R = crate::R<RAM_COMrs>;
#[doc = "Register `RAM_COM%s` writer"]
pub type W = crate::W<RAM_COMrs>;
#[doc = "Field `SEGS` reader - Segment states, one bit per segment, LSB: S00, MSB: S39"]
pub type SEGS_R = crate::FieldReader<u64>;
#[doc = "Field `SEGS` writer - Segment states, one bit per segment, LSB: S00, MSB: S39"]
pub type SEGS_W<'a, REG> = crate::FieldWriter<'a, REG, 40, u64>;
impl R {
    #[doc = "Bits 0:39 - Segment states, one bit per segment, LSB: S00, MSB: S39"]
    #[inline(always)]
    pub fn segs(&self) -> SEGS_R {
        SEGS_R::new(self.bits & 0x00ff_ffff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:39 - Segment states, one bit per segment, LSB: S00, MSB: S39"]
    #[inline(always)]
    #[must_use]
    pub fn segs(&mut self) -> SEGS_W<RAM_COMrs> {
        SEGS_W::new(self, 0)
    }
}
#[doc = "display memory\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ram_com::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ram_com::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RAM_COMrs;
impl crate::RegisterSpec for RAM_COMrs {
    type Ux = u64;
}
#[doc = "`read()` method returns [`ram_com::R`](R) reader structure"]
impl crate::Readable for RAM_COMrs {}
#[doc = "`write(|w| ..)` method takes [`ram_com::W`](W) writer structure"]
impl crate::Writable for RAM_COMrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets RAM_COM%s to value 0"]
impl crate::Resettable for RAM_COMrs {
    const RESET_VALUE: u64 = 0;
}
