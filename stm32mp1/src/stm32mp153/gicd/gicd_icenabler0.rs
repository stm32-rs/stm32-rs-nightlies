#[doc = "Register `GICD_ICENABLER0` reader"]
pub type R = crate::R<GICD_ICENABLER0rs>;
#[doc = "Register `GICD_ICENABLER0` writer"]
pub type W = crate::W<GICD_ICENABLER0rs>;
#[doc = "Field `ICENABLER0` reader - ICENABLER0"]
pub type ICENABLER0_R = crate::FieldReader<u32>;
#[doc = "Field `ICENABLER0` writer - ICENABLER0"]
pub type ICENABLER0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ICENABLER0"]
    #[inline(always)]
    pub fn icenabler0(&self) -> ICENABLER0_R {
        ICENABLER0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ICENABLER0"]
    #[inline(always)]
    #[must_use]
    pub fn icenabler0(&mut self) -> ICENABLER0_W<GICD_ICENABLER0rs> {
        ICENABLER0_W::new(self, 0)
    }
}
#[doc = "For interrupts ID = 0 to ID = 31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icenabler0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icenabler0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_ICENABLER0rs;
impl crate::RegisterSpec for GICD_ICENABLER0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_icenabler0::R`](R) reader structure"]
impl crate::Readable for GICD_ICENABLER0rs {}
#[doc = "`write(|w| ..)` method takes [`gicd_icenabler0::W`](W) writer structure"]
impl crate::Writable for GICD_ICENABLER0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ICENABLER0 to value 0xffff"]
impl crate::Resettable for GICD_ICENABLER0rs {
    const RESET_VALUE: u32 = 0xffff;
}
