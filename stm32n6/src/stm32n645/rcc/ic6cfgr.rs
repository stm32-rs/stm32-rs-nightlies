///Register `IC6CFGR` reader
pub type R = crate::R<IC6CFGRrs>;
///Register `IC6CFGR` writer
pub type W = crate::W<IC6CFGRrs>;
///Field `IC6INT` reader - Divider IC6 integer division factor
pub type IC6INT_R = crate::FieldReader;
///Field `IC6INT` writer - Divider IC6 integer division factor
pub type IC6INT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `IC6SEL` reader - Divider IC6 Source Selection
pub type IC6SEL_R = crate::FieldReader;
///Field `IC6SEL` writer - Divider IC6 Source Selection
pub type IC6SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 16:23 - Divider IC6 integer division factor
    #[inline(always)]
    pub fn ic6int(&self) -> IC6INT_R {
        IC6INT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 28:29 - Divider IC6 Source Selection
    #[inline(always)]
    pub fn ic6sel(&self) -> IC6SEL_R {
        IC6SEL_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IC6CFGR")
            .field("ic6int", &self.ic6int())
            .field("ic6sel", &self.ic6sel())
            .finish()
    }
}
impl W {
    ///Bits 16:23 - Divider IC6 integer division factor
    #[inline(always)]
    pub fn ic6int(&mut self) -> IC6INT_W<'_, IC6CFGRrs> {
        IC6INT_W::new(self, 16)
    }
    ///Bits 28:29 - Divider IC6 Source Selection
    #[inline(always)]
    pub fn ic6sel(&mut self) -> IC6SEL_W<'_, IC6CFGRrs> {
        IC6SEL_W::new(self, 28)
    }
}
/**RCC IC6 configuration register

You can [`read`](crate::Reg::read) this register and get [`ic6cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic6cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RCC:IC6CFGR)*/
pub struct IC6CFGRrs;
impl crate::RegisterSpec for IC6CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`ic6cfgr::R`](R) reader structure
impl crate::Readable for IC6CFGRrs {}
///`write(|w| ..)` method takes [`ic6cfgr::W`](W) writer structure
impl crate::Writable for IC6CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IC6CFGR to value 0x0003_0000
impl crate::Resettable for IC6CFGRrs {
    const RESET_VALUE: u32 = 0x0003_0000;
}
