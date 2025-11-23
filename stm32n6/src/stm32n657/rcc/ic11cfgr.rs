///Register `IC11CFGR` reader
pub type R = crate::R<IC11CFGRrs>;
///Register `IC11CFGR` writer
pub type W = crate::W<IC11CFGRrs>;
///Field `IC11INT` reader - Divider IC11 integer division factor
pub type IC11INT_R = crate::FieldReader;
///Field `IC11INT` writer - Divider IC11 integer division factor
pub type IC11INT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `IC11SEL` reader - Divider IC11 Source Selection
pub type IC11SEL_R = crate::FieldReader;
///Field `IC11SEL` writer - Divider IC11 Source Selection
pub type IC11SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 16:23 - Divider IC11 integer division factor
    #[inline(always)]
    pub fn ic11int(&self) -> IC11INT_R {
        IC11INT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 28:29 - Divider IC11 Source Selection
    #[inline(always)]
    pub fn ic11sel(&self) -> IC11SEL_R {
        IC11SEL_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IC11CFGR")
            .field("ic11int", &self.ic11int())
            .field("ic11sel", &self.ic11sel())
            .finish()
    }
}
impl W {
    ///Bits 16:23 - Divider IC11 integer division factor
    #[inline(always)]
    pub fn ic11int(&mut self) -> IC11INT_W<'_, IC11CFGRrs> {
        IC11INT_W::new(self, 16)
    }
    ///Bits 28:29 - Divider IC11 Source Selection
    #[inline(always)]
    pub fn ic11sel(&mut self) -> IC11SEL_W<'_, IC11CFGRrs> {
        IC11SEL_W::new(self, 28)
    }
}
/**RCC IC11 configuration register

You can [`read`](crate::Reg::read) this register and get [`ic11cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic11cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RCC:IC11CFGR)*/
pub struct IC11CFGRrs;
impl crate::RegisterSpec for IC11CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`ic11cfgr::R`](R) reader structure
impl crate::Readable for IC11CFGRrs {}
///`write(|w| ..)` method takes [`ic11cfgr::W`](W) writer structure
impl crate::Writable for IC11CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IC11CFGR to value 0x0003_0000
impl crate::Resettable for IC11CFGRrs {
    const RESET_VALUE: u32 = 0x0003_0000;
}
