///Register `IC7CFGR` reader
pub type R = crate::R<IC7CFGRrs>;
///Register `IC7CFGR` writer
pub type W = crate::W<IC7CFGRrs>;
///Field `IC7INT` reader - Divider IC7 integer division factor
pub type IC7INT_R = crate::FieldReader;
///Field `IC7INT` writer - Divider IC7 integer division factor
pub type IC7INT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `IC7SEL` reader - Divider IC7 Source Selection
pub type IC7SEL_R = crate::FieldReader;
///Field `IC7SEL` writer - Divider IC7 Source Selection
pub type IC7SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 16:23 - Divider IC7 integer division factor
    #[inline(always)]
    pub fn ic7int(&self) -> IC7INT_R {
        IC7INT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 28:29 - Divider IC7 Source Selection
    #[inline(always)]
    pub fn ic7sel(&self) -> IC7SEL_R {
        IC7SEL_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IC7CFGR")
            .field("ic7int", &self.ic7int())
            .field("ic7sel", &self.ic7sel())
            .finish()
    }
}
impl W {
    ///Bits 16:23 - Divider IC7 integer division factor
    #[inline(always)]
    pub fn ic7int(&mut self) -> IC7INT_W<'_, IC7CFGRrs> {
        IC7INT_W::new(self, 16)
    }
    ///Bits 28:29 - Divider IC7 Source Selection
    #[inline(always)]
    pub fn ic7sel(&mut self) -> IC7SEL_W<'_, IC7CFGRrs> {
        IC7SEL_W::new(self, 28)
    }
}
/**RCC IC7 configuration register

You can [`read`](crate::Reg::read) this register and get [`ic7cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic7cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RCC:IC7CFGR)*/
pub struct IC7CFGRrs;
impl crate::RegisterSpec for IC7CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`ic7cfgr::R`](R) reader structure
impl crate::Readable for IC7CFGRrs {}
///`write(|w| ..)` method takes [`ic7cfgr::W`](W) writer structure
impl crate::Writable for IC7CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IC7CFGR to value 0x1000_0000
impl crate::Resettable for IC7CFGRrs {
    const RESET_VALUE: u32 = 0x1000_0000;
}
