#[doc = "Register `PCROP1AER` reader"]
pub type R = crate::R<PCROP1AERrs>;
#[doc = "Register `PCROP1AER` writer"]
pub type W = crate::W<PCROP1AERrs>;
#[doc = "Field `PCROP1A_END` reader - PCROP area end offset"]
pub type PCROP1A_END_R = crate::FieldReader;
#[doc = "Field `PCROP1A_END` writer - PCROP area end offset"]
pub type PCROP1A_END_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
#[doc = "Field `PCROP_RDP` reader - PCROP area preserved when RDP level decreased"]
pub type PCROP_RDP_R = crate::BitReader;
#[doc = "Field `PCROP_RDP` writer - PCROP area preserved when RDP level decreased"]
pub type PCROP_RDP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PCROP area end offset"]
    #[inline(always)]
    pub fn pcrop1a_end(&self) -> PCROP1A_END_R {
        PCROP1A_END_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 31 - PCROP area preserved when RDP level decreased"]
    #[inline(always)]
    pub fn pcrop_rdp(&self) -> PCROP_RDP_R {
        PCROP_RDP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PCROP area end offset"]
    #[inline(always)]
    #[must_use]
    pub fn pcrop1a_end(&mut self) -> PCROP1A_END_W<PCROP1AERrs> {
        PCROP1A_END_W::new(self, 0)
    }
    #[doc = "Bit 31 - PCROP area preserved when RDP level decreased"]
    #[inline(always)]
    #[must_use]
    pub fn pcrop_rdp(&mut self) -> PCROP_RDP_W<PCROP1AERrs> {
        PCROP_RDP_W::new(self, 31)
    }
}
#[doc = "Flash PCROP zone A End address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcrop1aer::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcrop1aer::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCROP1AERrs;
impl crate::RegisterSpec for PCROP1AERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcrop1aer::R`](R) reader structure"]
impl crate::Readable for PCROP1AERrs {}
#[doc = "`write(|w| ..)` method takes [`pcrop1aer::W`](W) writer structure"]
impl crate::Writable for PCROP1AERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCROP1AER to value 0xffff_ff00"]
impl crate::Resettable for PCROP1AERrs {
    const RESET_VALUE: u32 = 0xffff_ff00;
}
