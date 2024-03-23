#[doc = "Register `FCCAN_CCU_IR` reader"]
pub type R = crate::R<FCCAN_CCU_IRrs>;
#[doc = "Register `FCCAN_CCU_IR` writer"]
pub type W = crate::W<FCCAN_CCU_IRrs>;
#[doc = "Field `CWE` reader - CWE"]
pub type CWE_R = crate::BitReader;
#[doc = "Field `CWE` writer - CWE"]
pub type CWE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSC` reader - CSC"]
pub type CSC_R = crate::BitReader;
#[doc = "Field `CSC` writer - CSC"]
pub type CSC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CWE"]
    #[inline(always)]
    pub fn cwe(&self) -> CWE_R {
        CWE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CSC"]
    #[inline(always)]
    pub fn csc(&self) -> CSC_R {
        CSC_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CWE"]
    #[inline(always)]
    #[must_use]
    pub fn cwe(&mut self) -> CWE_W<FCCAN_CCU_IRrs> {
        CWE_W::new(self, 0)
    }
    #[doc = "Bit 1 - CSC"]
    #[inline(always)]
    #[must_use]
    pub fn csc(&mut self) -> CSC_W<FCCAN_CCU_IRrs> {
        CSC_W::new(self, 1)
    }
}
#[doc = "The flags are set when one of the listed conditions is detected (edge-sensitive). The flags remain set until the Host clears them. A flag is cleared by writing a 1 to the corresponding bit position. Writing a 0 has no effect. A hard reset will clear the register. The configuration of CCU_IE controls whether an interrupt is generated or not.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fccan_ccu_ir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fccan_ccu_ir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FCCAN_CCU_IRrs;
impl crate::RegisterSpec for FCCAN_CCU_IRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fccan_ccu_ir::R`](R) reader structure"]
impl crate::Readable for FCCAN_CCU_IRrs {}
#[doc = "`write(|w| ..)` method takes [`fccan_ccu_ir::W`](W) writer structure"]
impl crate::Writable for FCCAN_CCU_IRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FCCAN_CCU_IR to value 0"]
impl crate::Resettable for FCCAN_CCU_IRrs {
    const RESET_VALUE: u32 = 0;
}
