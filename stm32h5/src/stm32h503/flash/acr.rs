#[doc = "Register `ACR` reader"]
pub type R = crate::R<ACRrs>;
#[doc = "Register `ACR` writer"]
pub type W = crate::W<ACRrs>;
#[doc = "Field `LATENCY` reader - Read latency These bits are used to control the number of wait states used during read operations on both non-volatile memory banks. The application software has to program them to the correct value depending on the embedded Flash memory interface frequency and voltage conditions. ... Note: No check is performed by hardware to verify that the configuration is correct."]
pub type LATENCY_R = crate::FieldReader;
#[doc = "Field `LATENCY` writer - Read latency These bits are used to control the number of wait states used during read operations on both non-volatile memory banks. The application software has to program them to the correct value depending on the embedded Flash memory interface frequency and voltage conditions. ... Note: No check is performed by hardware to verify that the configuration is correct."]
pub type LATENCY_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WRHIGHFREQ` reader - Flash signal delay These bits are used to control the delay between non-volatile memory signals during programming operations. Application software has to program them to the correct value depending on the embedded Flash memory interface frequency. Please refer to for details. Note: No check is performed to verify that the configuration is correct. Two WRHIGHFREQ values can be selected for some frequencies."]
pub type WRHIGHFREQ_R = crate::FieldReader;
#[doc = "Field `WRHIGHFREQ` writer - Flash signal delay These bits are used to control the delay between non-volatile memory signals during programming operations. Application software has to program them to the correct value depending on the embedded Flash memory interface frequency. Please refer to for details. Note: No check is performed to verify that the configuration is correct. Two WRHIGHFREQ values can be selected for some frequencies."]
pub type WRHIGHFREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRFTEN` reader - Prefetch enable. When bit value is modified, user must read back ACR register to be sure PRFTEN has been taken into account. Bits used to control the prefetch."]
pub type PRFTEN_R = crate::BitReader;
#[doc = "Field `PRFTEN` writer - Prefetch enable. When bit value is modified, user must read back ACR register to be sure PRFTEN has been taken into account. Bits used to control the prefetch."]
pub type PRFTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S_PRFTEN` reader - Smart prefetch enable. When bit value is modified, user must read back ACR register to be sure S_PRFTEN has been taken into account. Bits used to control the prefetch functionality."]
pub type S_PRFTEN_R = crate::BitReader;
#[doc = "Field `S_PRFTEN` writer - Smart prefetch enable. When bit value is modified, user must read back ACR register to be sure S_PRFTEN has been taken into account. Bits used to control the prefetch functionality."]
pub type S_PRFTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Read latency These bits are used to control the number of wait states used during read operations on both non-volatile memory banks. The application software has to program them to the correct value depending on the embedded Flash memory interface frequency and voltage conditions. ... Note: No check is performed by hardware to verify that the configuration is correct."]
    #[inline(always)]
    pub fn latency(&self) -> LATENCY_R {
        LATENCY_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Flash signal delay These bits are used to control the delay between non-volatile memory signals during programming operations. Application software has to program them to the correct value depending on the embedded Flash memory interface frequency. Please refer to for details. Note: No check is performed to verify that the configuration is correct. Two WRHIGHFREQ values can be selected for some frequencies."]
    #[inline(always)]
    pub fn wrhighfreq(&self) -> WRHIGHFREQ_R {
        WRHIGHFREQ_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - Prefetch enable. When bit value is modified, user must read back ACR register to be sure PRFTEN has been taken into account. Bits used to control the prefetch."]
    #[inline(always)]
    pub fn prften(&self) -> PRFTEN_R {
        PRFTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Smart prefetch enable. When bit value is modified, user must read back ACR register to be sure S_PRFTEN has been taken into account. Bits used to control the prefetch functionality."]
    #[inline(always)]
    pub fn s_prften(&self) -> S_PRFTEN_R {
        S_PRFTEN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Read latency These bits are used to control the number of wait states used during read operations on both non-volatile memory banks. The application software has to program them to the correct value depending on the embedded Flash memory interface frequency and voltage conditions. ... Note: No check is performed by hardware to verify that the configuration is correct."]
    #[inline(always)]
    #[must_use]
    pub fn latency(&mut self) -> LATENCY_W<ACRrs> {
        LATENCY_W::new(self, 0)
    }
    #[doc = "Bits 4:5 - Flash signal delay These bits are used to control the delay between non-volatile memory signals during programming operations. Application software has to program them to the correct value depending on the embedded Flash memory interface frequency. Please refer to for details. Note: No check is performed to verify that the configuration is correct. Two WRHIGHFREQ values can be selected for some frequencies."]
    #[inline(always)]
    #[must_use]
    pub fn wrhighfreq(&mut self) -> WRHIGHFREQ_W<ACRrs> {
        WRHIGHFREQ_W::new(self, 4)
    }
    #[doc = "Bit 8 - Prefetch enable. When bit value is modified, user must read back ACR register to be sure PRFTEN has been taken into account. Bits used to control the prefetch."]
    #[inline(always)]
    #[must_use]
    pub fn prften(&mut self) -> PRFTEN_W<ACRrs> {
        PRFTEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - Smart prefetch enable. When bit value is modified, user must read back ACR register to be sure S_PRFTEN has been taken into account. Bits used to control the prefetch functionality."]
    #[inline(always)]
    #[must_use]
    pub fn s_prften(&mut self) -> S_PRFTEN_W<ACRrs> {
        S_PRFTEN_W::new(self, 9)
    }
}
#[doc = "FLASH access control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ACRrs;
impl crate::RegisterSpec for ACRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acr::R`](R) reader structure"]
impl crate::Readable for ACRrs {}
#[doc = "`write(|w| ..)` method takes [`acr::W`](W) writer structure"]
impl crate::Writable for ACRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ACR to value 0x13"]
impl crate::Resettable for ACRrs {
    const RESET_VALUE: u32 = 0x13;
}
