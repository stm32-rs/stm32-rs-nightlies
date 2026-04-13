///Register `IC8CFGR` reader
pub type R = crate::R<IC8CFGRrs>;
///Register `IC8CFGR` writer
pub type W = crate::W<IC8CFGRrs>;
///Field `IC8INT` reader - Divider IC8 integer division factor
pub type IC8INT_R = crate::FieldReader;
///Field `IC8INT` writer - Divider IC8 integer division factor
pub type IC8INT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `IC8SEL` reader - Divider IC8 Source Selection
pub type IC8SEL_R = crate::FieldReader;
///Field `IC8SEL` writer - Divider IC8 Source Selection
pub type IC8SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 16:23 - Divider IC8 integer division factor
    #[inline(always)]
    pub fn ic8int(&self) -> IC8INT_R {
        IC8INT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 28:29 - Divider IC8 Source Selection
    #[inline(always)]
    pub fn ic8sel(&self) -> IC8SEL_R {
        IC8SEL_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IC8CFGR")
            .field("ic8int", &self.ic8int())
            .field("ic8sel", &self.ic8sel())
            .finish()
    }
}
impl W {
    ///Bits 16:23 - Divider IC8 integer division factor
    #[inline(always)]
    pub fn ic8int(&mut self) -> IC8INT_W<'_, IC8CFGRrs> {
        IC8INT_W::new(self, 16)
    }
    ///Bits 28:29 - Divider IC8 Source Selection
    #[inline(always)]
    pub fn ic8sel(&mut self) -> IC8SEL_W<'_, IC8CFGRrs> {
        IC8SEL_W::new(self, 28)
    }
}
/**RCC IC8 configuration register

You can [`read`](crate::Reg::read) this register and get [`ic8cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic8cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RCC:IC8CFGR)*/
pub struct IC8CFGRrs;
impl crate::RegisterSpec for IC8CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`ic8cfgr::R`](R) reader structure
impl crate::Readable for IC8CFGRrs {}
///`write(|w| ..)` method takes [`ic8cfgr::W`](W) writer structure
impl crate::Writable for IC8CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IC8CFGR to value 0x1000_0000
impl crate::Resettable for IC8CFGRrs {
    const RESET_VALUE: u32 = 0x1000_0000;
}
