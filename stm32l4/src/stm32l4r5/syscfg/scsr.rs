#[doc = "Register `SCSR` reader"]
pub type R = crate::R<SCSRrs>;
#[doc = "Register `SCSR` writer"]
pub type W = crate::W<SCSRrs>;
#[doc = "Field `SRAM2ER` reader - SRAM2 Erase"]
pub type SRAM2ER_R = crate::BitReader;
#[doc = "Field `SRAM2ER` writer - SRAM2 Erase"]
pub type SRAM2ER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM2BSY` reader - SRAM2 busy by erase operation"]
pub type SRAM2BSY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - SRAM2 Erase"]
    #[inline(always)]
    pub fn sram2er(&self) -> SRAM2ER_R {
        SRAM2ER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SRAM2 busy by erase operation"]
    #[inline(always)]
    pub fn sram2bsy(&self) -> SRAM2BSY_R {
        SRAM2BSY_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SRAM2 Erase"]
    #[inline(always)]
    #[must_use]
    pub fn sram2er(&mut self) -> SRAM2ER_W<SCSRrs> {
        SRAM2ER_W::new(self, 0)
    }
}
#[doc = "SCSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCSRrs;
impl crate::RegisterSpec for SCSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scsr::R`](R) reader structure"]
impl crate::Readable for SCSRrs {}
#[doc = "`write(|w| ..)` method takes [`scsr::W`](W) writer structure"]
impl crate::Writable for SCSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCSR to value 0"]
impl crate::Resettable for SCSRrs {
    const RESET_VALUE: u32 = 0;
}
