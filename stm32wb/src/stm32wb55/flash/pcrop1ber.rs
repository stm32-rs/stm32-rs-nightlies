#[doc = "Register `PCROP1BER` reader"]
pub type R = crate::R<PCROP1BERrs>;
#[doc = "Register `PCROP1BER` writer"]
pub type W = crate::W<PCROP1BERrs>;
#[doc = "Field `PCROP1B_END` reader - Bank 1 PCROP area end area B offset"]
pub type PCROP1B_END_R = crate::FieldReader<u16>;
#[doc = "Field `PCROP1B_END` writer - Bank 1 PCROP area end area B offset"]
pub type PCROP1B_END_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - Bank 1 PCROP area end area B offset"]
    #[inline(always)]
    pub fn pcrop1b_end(&self) -> PCROP1B_END_R {
        PCROP1B_END_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Bank 1 PCROP area end area B offset"]
    #[inline(always)]
    #[must_use]
    pub fn pcrop1b_end(&mut self) -> PCROP1B_END_W<PCROP1BERrs> {
        PCROP1B_END_W::new(self, 0)
    }
}
#[doc = "Flash Bank 1 PCROP End address area B register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcrop1ber::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcrop1ber::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCROP1BERrs;
impl crate::RegisterSpec for PCROP1BERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcrop1ber::R`](R) reader structure"]
impl crate::Readable for PCROP1BERrs {}
#[doc = "`write(|w| ..)` method takes [`pcrop1ber::W`](W) writer structure"]
impl crate::Writable for PCROP1BERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCROP1BER to value 0xffff_fe00"]
impl crate::Resettable for PCROP1BERrs {
    const RESET_VALUE: u32 = 0xffff_fe00;
}
