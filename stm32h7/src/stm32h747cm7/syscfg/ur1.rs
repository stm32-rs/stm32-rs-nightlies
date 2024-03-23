#[doc = "Register `UR1` reader"]
pub type R = crate::R<UR1rs>;
#[doc = "Register `UR1` writer"]
pub type W = crate::W<UR1rs>;
#[doc = "Field `BCM4` reader - Boot Cortex-M4"]
pub type BCM4_R = crate::BitReader;
#[doc = "Field `BCM4` writer - Boot Cortex-M4"]
pub type BCM4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BCM7` reader - Boot Cortex-M7"]
pub type BCM7_R = crate::BitReader;
#[doc = "Field `BCM7` writer - Boot Cortex-M7"]
pub type BCM7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Boot Cortex-M4"]
    #[inline(always)]
    pub fn bcm4(&self) -> BCM4_R {
        BCM4_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - Boot Cortex-M7"]
    #[inline(always)]
    pub fn bcm7(&self) -> BCM7_R {
        BCM7_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Boot Cortex-M4"]
    #[inline(always)]
    #[must_use]
    pub fn bcm4(&mut self) -> BCM4_W<UR1rs> {
        BCM4_W::new(self, 0)
    }
    #[doc = "Bit 16 - Boot Cortex-M7"]
    #[inline(always)]
    #[must_use]
    pub fn bcm7(&mut self) -> BCM7_W<UR1rs> {
        BCM7_W::new(self, 16)
    }
}
#[doc = "SYSCFG user register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ur1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ur1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UR1rs;
impl crate::RegisterSpec for UR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ur1::R`](R) reader structure"]
impl crate::Readable for UR1rs {}
#[doc = "`write(|w| ..)` method takes [`ur1::W`](W) writer structure"]
impl crate::Writable for UR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UR1 to value 0"]
impl crate::Resettable for UR1rs {
    const RESET_VALUE: u32 = 0;
}
