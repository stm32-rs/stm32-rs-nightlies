#[doc = "Register `UR4` reader"]
pub type R = crate::R<UR4rs>;
#[doc = "Register `UR4` writer"]
pub type W = crate::W<UR4rs>;
#[doc = "Field `BCM4_ADD1` reader - Mass Erase Protected Area Disabled for bank 1"]
pub type BCM4_ADD1_R = crate::FieldReader<u16>;
#[doc = "Field `BCM4_ADD1` writer - Mass Erase Protected Area Disabled for bank 1"]
pub type BCM4_ADD1_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MEPAD_1` reader - Boot Cortex-M4 Address 1"]
pub type MEPAD_1_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - Mass Erase Protected Area Disabled for bank 1"]
    #[inline(always)]
    pub fn bcm4_add1(&self) -> BCM4_ADD1_R {
        BCM4_ADD1_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Boot Cortex-M4 Address 1"]
    #[inline(always)]
    pub fn mepad_1(&self) -> MEPAD_1_R {
        MEPAD_1_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Mass Erase Protected Area Disabled for bank 1"]
    #[inline(always)]
    #[must_use]
    pub fn bcm4_add1(&mut self) -> BCM4_ADD1_W<UR4rs> {
        BCM4_ADD1_W::new(self, 0)
    }
}
#[doc = "SYSCFG user register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ur4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ur4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UR4rs;
impl crate::RegisterSpec for UR4rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ur4::R`](R) reader structure"]
impl crate::Readable for UR4rs {}
#[doc = "`write(|w| ..)` method takes [`ur4::W`](W) writer structure"]
impl crate::Writable for UR4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UR4 to value 0"]
impl crate::Resettable for UR4rs {
    const RESET_VALUE: u32 = 0;
}
