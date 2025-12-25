///Register `IC10CFGR` reader
pub type R = crate::R<IC10CFGRrs>;
///Register `IC10CFGR` writer
pub type W = crate::W<IC10CFGRrs>;
///Field `IC10INT` reader - Divider IC10 integer division factor
pub type IC10INT_R = crate::FieldReader;
///Field `IC10INT` writer - Divider IC10 integer division factor
pub type IC10INT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `IC10SEL` reader - Divider IC10 Source Selection
pub type IC10SEL_R = crate::FieldReader;
///Field `IC10SEL` writer - Divider IC10 Source Selection
pub type IC10SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 16:23 - Divider IC10 integer division factor
    #[inline(always)]
    pub fn ic10int(&self) -> IC10INT_R {
        IC10INT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 28:29 - Divider IC10 Source Selection
    #[inline(always)]
    pub fn ic10sel(&self) -> IC10SEL_R {
        IC10SEL_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IC10CFGR")
            .field("ic10int", &self.ic10int())
            .field("ic10sel", &self.ic10sel())
            .finish()
    }
}
impl W {
    ///Bits 16:23 - Divider IC10 integer division factor
    #[inline(always)]
    pub fn ic10int(&mut self) -> IC10INT_W<'_, IC10CFGRrs> {
        IC10INT_W::new(self, 16)
    }
    ///Bits 28:29 - Divider IC10 Source Selection
    #[inline(always)]
    pub fn ic10sel(&mut self) -> IC10SEL_W<'_, IC10CFGRrs> {
        IC10SEL_W::new(self, 28)
    }
}
/**RCC IC10 configuration register

You can [`read`](crate::Reg::read) this register and get [`ic10cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic10cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RCC:IC10CFGR)*/
pub struct IC10CFGRrs;
impl crate::RegisterSpec for IC10CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`ic10cfgr::R`](R) reader structure
impl crate::Readable for IC10CFGRrs {}
///`write(|w| ..)` method takes [`ic10cfgr::W`](W) writer structure
impl crate::Writable for IC10CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IC10CFGR to value 0x1000_0000
impl crate::Resettable for IC10CFGRrs {
    const RESET_VALUE: u32 = 0x1000_0000;
}
