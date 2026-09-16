///Register `IC9CFGR` reader
pub type R = crate::R<IC9CFGRrs>;
///Register `IC9CFGR` writer
pub type W = crate::W<IC9CFGRrs>;
///Field `IC9INT` reader - Divider IC9 integer division factor
pub type IC9INT_R = crate::FieldReader;
///Field `IC9INT` writer - Divider IC9 integer division factor
pub type IC9INT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `IC9SEL` reader - Divider IC9 Source Selection
pub type IC9SEL_R = crate::FieldReader;
///Field `IC9SEL` writer - Divider IC9 Source Selection
pub type IC9SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 16:23 - Divider IC9 integer division factor
    #[inline(always)]
    pub fn ic9int(&self) -> IC9INT_R {
        IC9INT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 28:29 - Divider IC9 Source Selection
    #[inline(always)]
    pub fn ic9sel(&self) -> IC9SEL_R {
        IC9SEL_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IC9CFGR")
            .field("ic9int", &self.ic9int())
            .field("ic9sel", &self.ic9sel())
            .finish()
    }
}
impl W {
    ///Bits 16:23 - Divider IC9 integer division factor
    #[inline(always)]
    pub fn ic9int(&mut self) -> IC9INT_W<'_, IC9CFGRrs> {
        IC9INT_W::new(self, 16)
    }
    ///Bits 28:29 - Divider IC9 Source Selection
    #[inline(always)]
    pub fn ic9sel(&mut self) -> IC9SEL_W<'_, IC9CFGRrs> {
        IC9SEL_W::new(self, 28)
    }
}
/**RCC IC9 configuration register

You can [`read`](crate::Reg::read) this register and get [`ic9cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic9cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RCC:IC9CFGR)*/
pub struct IC9CFGRrs;
impl crate::RegisterSpec for IC9CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`ic9cfgr::R`](R) reader structure
impl crate::Readable for IC9CFGRrs {}
///`write(|w| ..)` method takes [`ic9cfgr::W`](W) writer structure
impl crate::Writable for IC9CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IC9CFGR to value 0x1000_0000
impl crate::Resettable for IC9CFGRrs {
    const RESET_VALUE: u32 = 0x1000_0000;
}
