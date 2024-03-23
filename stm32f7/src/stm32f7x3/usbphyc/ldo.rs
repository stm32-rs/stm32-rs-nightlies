#[doc = "Register `LDO` reader"]
pub type R = crate::R<LDOrs>;
#[doc = "Register `LDO` writer"]
pub type W = crate::W<LDOrs>;
#[doc = "Field `LDO_USED` reader - Indicates the presence of the LDO in the chip"]
pub type LDO_USED_R = crate::BitReader;
#[doc = "Field `LDO_STATUS` reader - Monitors the status of the PHY's LDO"]
pub type LDO_STATUS_R = crate::BitReader;
#[doc = "Field `LDO_DISABLE` reader - Controls disable of the High Speed PHY's LDO"]
pub type LDO_DISABLE_R = crate::BitReader;
#[doc = "Field `LDO_DISABLE` writer - Controls disable of the High Speed PHY's LDO"]
pub type LDO_DISABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Indicates the presence of the LDO in the chip"]
    #[inline(always)]
    pub fn ldo_used(&self) -> LDO_USED_R {
        LDO_USED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Monitors the status of the PHY's LDO"]
    #[inline(always)]
    pub fn ldo_status(&self) -> LDO_STATUS_R {
        LDO_STATUS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Controls disable of the High Speed PHY's LDO"]
    #[inline(always)]
    pub fn ldo_disable(&self) -> LDO_DISABLE_R {
        LDO_DISABLE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Controls disable of the High Speed PHY's LDO"]
    #[inline(always)]
    #[must_use]
    pub fn ldo_disable(&mut self) -> LDO_DISABLE_W<LDOrs> {
        LDO_DISABLE_W::new(self, 2)
    }
}
#[doc = "USBPHYC LDO control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ldo::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ldo::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LDOrs;
impl crate::RegisterSpec for LDOrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ldo::R`](R) reader structure"]
impl crate::Readable for LDOrs {}
#[doc = "`write(|w| ..)` method takes [`ldo::W`](W) writer structure"]
impl crate::Writable for LDOrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LDO to value 0x01"]
impl crate::Resettable for LDOrs {
    const RESET_VALUE: u32 = 0x01;
}
