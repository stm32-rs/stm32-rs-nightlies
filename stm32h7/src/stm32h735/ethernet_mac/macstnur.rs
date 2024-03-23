#[doc = "Register `MACSTNUR` reader"]
pub type R = crate::R<MACSTNURrs>;
#[doc = "Register `MACSTNUR` writer"]
pub type W = crate::W<MACSTNURrs>;
#[doc = "Field `TSSS` reader - Timestamp Sub-seconds"]
pub type TSSS_R = crate::FieldReader<u32>;
#[doc = "Field `TSSS` writer - Timestamp Sub-seconds"]
pub type TSSS_W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
#[doc = "Field `ADDSUB` reader - Add or Subtract Time"]
pub type ADDSUB_R = crate::BitReader;
#[doc = "Field `ADDSUB` writer - Add or Subtract Time"]
pub type ADDSUB_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:30 - Timestamp Sub-seconds"]
    #[inline(always)]
    pub fn tsss(&self) -> TSSS_R {
        TSSS_R::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - Add or Subtract Time"]
    #[inline(always)]
    pub fn addsub(&self) -> ADDSUB_R {
        ADDSUB_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - Timestamp Sub-seconds"]
    #[inline(always)]
    #[must_use]
    pub fn tsss(&mut self) -> TSSS_W<MACSTNURrs> {
        TSSS_W::new(self, 0)
    }
    #[doc = "Bit 31 - Add or Subtract Time"]
    #[inline(always)]
    #[must_use]
    pub fn addsub(&mut self) -> ADDSUB_W<MACSTNURrs> {
        ADDSUB_W::new(self, 31)
    }
}
#[doc = "System time nanoseconds update register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macstnur::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macstnur::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACSTNURrs;
impl crate::RegisterSpec for MACSTNURrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macstnur::R`](R) reader structure"]
impl crate::Readable for MACSTNURrs {}
#[doc = "`write(|w| ..)` method takes [`macstnur::W`](W) writer structure"]
impl crate::Writable for MACSTNURrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACSTNUR to value 0"]
impl crate::Resettable for MACSTNURrs {
    const RESET_VALUE: u32 = 0;
}
