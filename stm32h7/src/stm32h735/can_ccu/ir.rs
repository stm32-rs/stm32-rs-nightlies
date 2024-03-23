#[doc = "Register `IR` reader"]
pub type R = crate::R<IRrs>;
#[doc = "Register `IR` writer"]
pub type W = crate::W<IRrs>;
#[doc = "Field `CWE` reader - Calibration Watchdog Event"]
pub type CWE_R = crate::BitReader;
#[doc = "Field `CWE` writer - Calibration Watchdog Event"]
pub type CWE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSC` reader - Calibration State Changed"]
pub type CSC_R = crate::BitReader;
#[doc = "Field `CSC` writer - Calibration State Changed"]
pub type CSC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Calibration Watchdog Event"]
    #[inline(always)]
    pub fn cwe(&self) -> CWE_R {
        CWE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Calibration State Changed"]
    #[inline(always)]
    pub fn csc(&self) -> CSC_R {
        CSC_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Calibration Watchdog Event"]
    #[inline(always)]
    #[must_use]
    pub fn cwe(&mut self) -> CWE_W<IRrs> {
        CWE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Calibration State Changed"]
    #[inline(always)]
    #[must_use]
    pub fn csc(&mut self) -> CSC_W<IRrs> {
        CSC_W::new(self, 1)
    }
}
#[doc = "Clock Calibration Unit Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IRrs;
impl crate::RegisterSpec for IRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ir::R`](R) reader structure"]
impl crate::Readable for IRrs {}
#[doc = "`write(|w| ..)` method takes [`ir::W`](W) writer structure"]
impl crate::Writable for IRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IR to value 0"]
impl crate::Resettable for IRrs {
    const RESET_VALUE: u32 = 0;
}
