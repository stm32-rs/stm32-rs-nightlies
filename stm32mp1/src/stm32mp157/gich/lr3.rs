///Register `LR3` reader
pub type R = crate::R<LR3rs>;
///Register `LR3` writer
pub type W = crate::W<LR3rs>;
///Field `VIRTUALID` reader - VIRTUALID
pub type VIRTUALID_R = crate::FieldReader<u16>;
///Field `VIRTUALID` writer - VIRTUALID
pub type VIRTUALID_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `PHYSICALID` reader - PHYSICALID
pub type PHYSICALID_R = crate::FieldReader<u16>;
///Field `PHYSICALID` writer - PHYSICALID
pub type PHYSICALID_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `PRIORITY` reader - PRIORITY
pub type PRIORITY_R = crate::FieldReader;
///Field `PRIORITY` writer - PRIORITY
pub type PRIORITY_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `STATE` reader - STATE
pub type STATE_R = crate::FieldReader;
///Field `STATE` writer - STATE
pub type STATE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `GRP1` reader - GRP1
pub type GRP1_R = crate::BitReader;
///Field `GRP1` writer - GRP1
pub type GRP1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HW` reader - HW
pub type HW_R = crate::BitReader;
///Field `HW` writer - HW
pub type HW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:9 - VIRTUALID
    #[inline(always)]
    pub fn virtualid(&self) -> VIRTUALID_R {
        VIRTUALID_R::new((self.bits & 0x03ff) as u16)
    }
    ///Bits 10:19 - PHYSICALID
    #[inline(always)]
    pub fn physicalid(&self) -> PHYSICALID_R {
        PHYSICALID_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    ///Bits 23:27 - PRIORITY
    #[inline(always)]
    pub fn priority(&self) -> PRIORITY_R {
        PRIORITY_R::new(((self.bits >> 23) & 0x1f) as u8)
    }
    ///Bits 28:29 - STATE
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bit 30 - GRP1
    #[inline(always)]
    pub fn grp1(&self) -> GRP1_R {
        GRP1_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - HW
    #[inline(always)]
    pub fn hw(&self) -> HW_R {
        HW_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LR3")
            .field("virtualid", &self.virtualid())
            .field("physicalid", &self.physicalid())
            .field("priority", &self.priority())
            .field("state", &self.state())
            .field("grp1", &self.grp1())
            .field("hw", &self.hw())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - VIRTUALID
    #[inline(always)]
    pub fn virtualid(&mut self) -> VIRTUALID_W<'_, LR3rs> {
        VIRTUALID_W::new(self, 0)
    }
    ///Bits 10:19 - PHYSICALID
    #[inline(always)]
    pub fn physicalid(&mut self) -> PHYSICALID_W<'_, LR3rs> {
        PHYSICALID_W::new(self, 10)
    }
    ///Bits 23:27 - PRIORITY
    #[inline(always)]
    pub fn priority(&mut self) -> PRIORITY_W<'_, LR3rs> {
        PRIORITY_W::new(self, 23)
    }
    ///Bits 28:29 - STATE
    #[inline(always)]
    pub fn state(&mut self) -> STATE_W<'_, LR3rs> {
        STATE_W::new(self, 28)
    }
    ///Bit 30 - GRP1
    #[inline(always)]
    pub fn grp1(&mut self) -> GRP1_W<'_, LR3rs> {
        GRP1_W::new(self, 30)
    }
    ///Bit 31 - HW
    #[inline(always)]
    pub fn hw(&mut self) -> HW_W<'_, LR3rs> {
        HW_W::new(self, 31)
    }
}
/**GICH list register 3

You can [`read`](crate::Reg::read) this register and get [`lr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICH:LR3)*/
pub struct LR3rs;
impl crate::RegisterSpec for LR3rs {
    type Ux = u32;
}
///`read()` method returns [`lr3::R`](R) reader structure
impl crate::Readable for LR3rs {}
///`write(|w| ..)` method takes [`lr3::W`](W) writer structure
impl crate::Writable for LR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LR3 to value 0
impl crate::Resettable for LR3rs {}
