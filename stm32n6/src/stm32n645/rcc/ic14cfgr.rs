///Register `IC14CFGR` reader
pub type R = crate::R<IC14CFGRrs>;
///Register `IC14CFGR` writer
pub type W = crate::W<IC14CFGRrs>;
///Field `IC14INT` reader - Divider IC14 integer division factor
pub type IC14INT_R = crate::FieldReader;
///Field `IC14INT` writer - Divider IC14 integer division factor
pub type IC14INT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `IC14SEL` reader - Divider IC14 Source Selection
pub type IC14SEL_R = crate::FieldReader;
///Field `IC14SEL` writer - Divider IC14 Source Selection
pub type IC14SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 16:23 - Divider IC14 integer division factor
    #[inline(always)]
    pub fn ic14int(&self) -> IC14INT_R {
        IC14INT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 28:29 - Divider IC14 Source Selection
    #[inline(always)]
    pub fn ic14sel(&self) -> IC14SEL_R {
        IC14SEL_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IC14CFGR")
            .field("ic14int", &self.ic14int())
            .field("ic14sel", &self.ic14sel())
            .finish()
    }
}
impl W {
    ///Bits 16:23 - Divider IC14 integer division factor
    #[inline(always)]
    pub fn ic14int(&mut self) -> IC14INT_W<'_, IC14CFGRrs> {
        IC14INT_W::new(self, 16)
    }
    ///Bits 28:29 - Divider IC14 Source Selection
    #[inline(always)]
    pub fn ic14sel(&mut self) -> IC14SEL_W<'_, IC14CFGRrs> {
        IC14SEL_W::new(self, 28)
    }
}
/**RCC IC14 configuration register

You can [`read`](crate::Reg::read) this register and get [`ic14cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic14cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RCC:IC14CFGR)*/
pub struct IC14CFGRrs;
impl crate::RegisterSpec for IC14CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`ic14cfgr::R`](R) reader structure
impl crate::Readable for IC14CFGRrs {}
///`write(|w| ..)` method takes [`ic14cfgr::W`](W) writer structure
impl crate::Writable for IC14CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IC14CFGR to value 0x2000_0000
impl crate::Resettable for IC14CFGRrs {
    const RESET_VALUE: u32 = 0x2000_0000;
}
