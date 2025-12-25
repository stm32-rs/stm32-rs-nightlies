///Register `IC2CFGR` reader
pub type R = crate::R<IC2CFGRrs>;
///Register `IC2CFGR` writer
pub type W = crate::W<IC2CFGRrs>;
///Field `IC2INT` reader - Divider IC2 integer division factor
pub type IC2INT_R = crate::FieldReader;
///Field `IC2INT` writer - Divider IC2 integer division factor
pub type IC2INT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `IC2SEL` reader - Divider IC2 Source Selection
pub type IC2SEL_R = crate::FieldReader;
///Field `IC2SEL` writer - Divider IC2 Source Selection
pub type IC2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 16:23 - Divider IC2 integer division factor
    #[inline(always)]
    pub fn ic2int(&self) -> IC2INT_R {
        IC2INT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 28:29 - Divider IC2 Source Selection
    #[inline(always)]
    pub fn ic2sel(&self) -> IC2SEL_R {
        IC2SEL_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IC2CFGR")
            .field("ic2int", &self.ic2int())
            .field("ic2sel", &self.ic2sel())
            .finish()
    }
}
impl W {
    ///Bits 16:23 - Divider IC2 integer division factor
    #[inline(always)]
    pub fn ic2int(&mut self) -> IC2INT_W<'_, IC2CFGRrs> {
        IC2INT_W::new(self, 16)
    }
    ///Bits 28:29 - Divider IC2 Source Selection
    #[inline(always)]
    pub fn ic2sel(&mut self) -> IC2SEL_W<'_, IC2CFGRrs> {
        IC2SEL_W::new(self, 28)
    }
}
/**RCC IC2 configuration register

You can [`read`](crate::Reg::read) this register and get [`ic2cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic2cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RCC:IC2CFGR)*/
pub struct IC2CFGRrs;
impl crate::RegisterSpec for IC2CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`ic2cfgr::R`](R) reader structure
impl crate::Readable for IC2CFGRrs {}
///`write(|w| ..)` method takes [`ic2cfgr::W`](W) writer structure
impl crate::Writable for IC2CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IC2CFGR to value 0x0003_0000
impl crate::Resettable for IC2CFGRrs {
    const RESET_VALUE: u32 = 0x0003_0000;
}
