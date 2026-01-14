///Register `IC17CFGR` reader
pub type R = crate::R<IC17CFGRrs>;
///Register `IC17CFGR` writer
pub type W = crate::W<IC17CFGRrs>;
///Field `IC17INT` reader - Divider IC17 integer division factor
pub type IC17INT_R = crate::FieldReader;
///Field `IC17INT` writer - Divider IC17 integer division factor
pub type IC17INT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `IC17SEL` reader - Divider IC17 Source Selection
pub type IC17SEL_R = crate::FieldReader;
///Field `IC17SEL` writer - Divider IC17 Source Selection
pub type IC17SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 16:23 - Divider IC17 integer division factor
    #[inline(always)]
    pub fn ic17int(&self) -> IC17INT_R {
        IC17INT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 28:29 - Divider IC17 Source Selection
    #[inline(always)]
    pub fn ic17sel(&self) -> IC17SEL_R {
        IC17SEL_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IC17CFGR")
            .field("ic17int", &self.ic17int())
            .field("ic17sel", &self.ic17sel())
            .finish()
    }
}
impl W {
    ///Bits 16:23 - Divider IC17 integer division factor
    #[inline(always)]
    pub fn ic17int(&mut self) -> IC17INT_W<'_, IC17CFGRrs> {
        IC17INT_W::new(self, 16)
    }
    ///Bits 28:29 - Divider IC17 Source Selection
    #[inline(always)]
    pub fn ic17sel(&mut self) -> IC17SEL_W<'_, IC17CFGRrs> {
        IC17SEL_W::new(self, 28)
    }
}
/**RCC IC17 configuration register

You can [`read`](crate::Reg::read) this register and get [`ic17cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic17cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RCC:IC17CFGR)*/
pub struct IC17CFGRrs;
impl crate::RegisterSpec for IC17CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`ic17cfgr::R`](R) reader structure
impl crate::Readable for IC17CFGRrs {}
///`write(|w| ..)` method takes [`ic17cfgr::W`](W) writer structure
impl crate::Writable for IC17CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IC17CFGR to value 0x3000_0000
impl crate::Resettable for IC17CFGRrs {
    const RESET_VALUE: u32 = 0x3000_0000;
}
