#[doc = "Register `UR2` reader"]
pub type R = crate::R<UR2rs>;
#[doc = "Register `UR2` writer"]
pub type W = crate::W<UR2rs>;
#[doc = "Field `BORH` reader - BOR_LVL Brownout Reset Threshold Level"]
pub type BORH_R = crate::FieldReader;
#[doc = "Field `BORH` writer - BOR_LVL Brownout Reset Threshold Level"]
pub type BORH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BOOT_ADD0` reader - Boot Address 0"]
pub type BOOT_ADD0_R = crate::FieldReader<u16>;
#[doc = "Field `BOOT_ADD0` writer - Boot Address 0"]
pub type BOOT_ADD0_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - BOR_LVL Brownout Reset Threshold Level"]
    #[inline(always)]
    pub fn borh(&self) -> BORH_R {
        BORH_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:31 - Boot Address 0"]
    #[inline(always)]
    pub fn boot_add0(&self) -> BOOT_ADD0_R {
        BOOT_ADD0_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - BOR_LVL Brownout Reset Threshold Level"]
    #[inline(always)]
    #[must_use]
    pub fn borh(&mut self) -> BORH_W<UR2rs> {
        BORH_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Boot Address 0"]
    #[inline(always)]
    #[must_use]
    pub fn boot_add0(&mut self) -> BOOT_ADD0_W<UR2rs> {
        BOOT_ADD0_W::new(self, 16)
    }
}
#[doc = "SYSCFG user register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ur2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ur2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UR2rs;
impl crate::RegisterSpec for UR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ur2::R`](R) reader structure"]
impl crate::Readable for UR2rs {}
#[doc = "`write(|w| ..)` method takes [`ur2::W`](W) writer structure"]
impl crate::Writable for UR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UR2 to value 0"]
impl crate::Resettable for UR2rs {
    const RESET_VALUE: u32 = 0;
}
