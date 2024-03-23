#[doc = "Register `SCSR` reader"]
pub type R = crate::R<SCSRrs>;
#[doc = "Register `SCSR` writer"]
pub type W = crate::W<SCSRrs>;
#[doc = "Field `CCMER` reader - CCM SRAM Erase"]
pub type CCMER_R = crate::BitReader;
#[doc = "Field `CCMER` writer - CCM SRAM Erase"]
pub type CCMER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCMBSY` reader - CCM SRAM busy by erase operation"]
pub type CCMBSY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - CCM SRAM Erase"]
    #[inline(always)]
    pub fn ccmer(&self) -> CCMER_R {
        CCMER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CCM SRAM busy by erase operation"]
    #[inline(always)]
    pub fn ccmbsy(&self) -> CCMBSY_R {
        CCMBSY_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CCM SRAM Erase"]
    #[inline(always)]
    #[must_use]
    pub fn ccmer(&mut self) -> CCMER_W<SCSRrs> {
        CCMER_W::new(self, 0)
    }
}
#[doc = "CCM SRAM control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
