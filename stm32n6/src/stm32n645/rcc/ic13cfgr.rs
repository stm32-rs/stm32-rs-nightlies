///Register `IC13CFGR` reader
pub type R = crate::R<IC13CFGRrs>;
///Register `IC13CFGR` writer
pub type W = crate::W<IC13CFGRrs>;
///Field `IC13INT` reader - Divider IC13 integer division factor
pub type IC13INT_R = crate::FieldReader;
///Field `IC13INT` writer - Divider IC13 integer division factor
pub type IC13INT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `IC13SEL` reader - Divider IC13 Source Selection
pub type IC13SEL_R = crate::FieldReader;
///Field `IC13SEL` writer - Divider IC13 Source Selection
pub type IC13SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 16:23 - Divider IC13 integer division factor
    #[inline(always)]
    pub fn ic13int(&self) -> IC13INT_R {
        IC13INT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 28:29 - Divider IC13 Source Selection
    #[inline(always)]
    pub fn ic13sel(&self) -> IC13SEL_R {
        IC13SEL_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IC13CFGR")
            .field("ic13int", &self.ic13int())
            .field("ic13sel", &self.ic13sel())
            .finish()
    }
}
impl W {
    ///Bits 16:23 - Divider IC13 integer division factor
    #[inline(always)]
    pub fn ic13int(&mut self) -> IC13INT_W<'_, IC13CFGRrs> {
        IC13INT_W::new(self, 16)
    }
    ///Bits 28:29 - Divider IC13 Source Selection
    #[inline(always)]
    pub fn ic13sel(&mut self) -> IC13SEL_W<'_, IC13CFGRrs> {
        IC13SEL_W::new(self, 28)
    }
}
/**RCC IC13 configuration register

You can [`read`](crate::Reg::read) this register and get [`ic13cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic13cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RCC:IC13CFGR)*/
pub struct IC13CFGRrs;
impl crate::RegisterSpec for IC13CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`ic13cfgr::R`](R) reader structure
impl crate::Readable for IC13CFGRrs {}
///`write(|w| ..)` method takes [`ic13cfgr::W`](W) writer structure
impl crate::Writable for IC13CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IC13CFGR to value 0x2000_0000
impl crate::Resettable for IC13CFGRrs {
    const RESET_VALUE: u32 = 0x2000_0000;
}
