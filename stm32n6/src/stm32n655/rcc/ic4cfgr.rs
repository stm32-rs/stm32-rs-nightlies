///Register `IC4CFGR` reader
pub type R = crate::R<IC4CFGRrs>;
///Register `IC4CFGR` writer
pub type W = crate::W<IC4CFGRrs>;
///Field `IC4INT` reader - Divider IC4 integer division factor
pub type IC4INT_R = crate::FieldReader;
///Field `IC4INT` writer - Divider IC4 integer division factor
pub type IC4INT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `IC4SEL` reader - Divider IC4 Source Selection
pub type IC4SEL_R = crate::FieldReader;
///Field `IC4SEL` writer - Divider IC4 Source Selection
pub type IC4SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 16:23 - Divider IC4 integer division factor
    #[inline(always)]
    pub fn ic4int(&self) -> IC4INT_R {
        IC4INT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 28:29 - Divider IC4 Source Selection
    #[inline(always)]
    pub fn ic4sel(&self) -> IC4SEL_R {
        IC4SEL_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IC4CFGR")
            .field("ic4int", &self.ic4int())
            .field("ic4sel", &self.ic4sel())
            .finish()
    }
}
impl W {
    ///Bits 16:23 - Divider IC4 integer division factor
    #[inline(always)]
    pub fn ic4int(&mut self) -> IC4INT_W<'_, IC4CFGRrs> {
        IC4INT_W::new(self, 16)
    }
    ///Bits 28:29 - Divider IC4 Source Selection
    #[inline(always)]
    pub fn ic4sel(&mut self) -> IC4SEL_W<'_, IC4CFGRrs> {
        IC4SEL_W::new(self, 28)
    }
}
/**RCC IC4 configuration register

You can [`read`](crate::Reg::read) this register and get [`ic4cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic4cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RCC:IC4CFGR)*/
pub struct IC4CFGRrs;
impl crate::RegisterSpec for IC4CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`ic4cfgr::R`](R) reader structure
impl crate::Readable for IC4CFGRrs {}
///`write(|w| ..)` method takes [`ic4cfgr::W`](W) writer structure
impl crate::Writable for IC4CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IC4CFGR to value 0
impl crate::Resettable for IC4CFGRrs {}
