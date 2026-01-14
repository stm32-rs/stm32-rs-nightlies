///Register `IC19CFGR` reader
pub type R = crate::R<IC19CFGRrs>;
///Register `IC19CFGR` writer
pub type W = crate::W<IC19CFGRrs>;
///Field `IC19INT` reader - Divider IC19 integer division factor
pub type IC19INT_R = crate::FieldReader;
///Field `IC19INT` writer - Divider IC19 integer division factor
pub type IC19INT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `IC19SEL` reader - Divider IC19 Source Selection
pub type IC19SEL_R = crate::FieldReader;
///Field `IC19SEL` writer - Divider IC19 Source Selection
pub type IC19SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 16:23 - Divider IC19 integer division factor
    #[inline(always)]
    pub fn ic19int(&self) -> IC19INT_R {
        IC19INT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 28:29 - Divider IC19 Source Selection
    #[inline(always)]
    pub fn ic19sel(&self) -> IC19SEL_R {
        IC19SEL_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IC19CFGR")
            .field("ic19int", &self.ic19int())
            .field("ic19sel", &self.ic19sel())
            .finish()
    }
}
impl W {
    ///Bits 16:23 - Divider IC19 integer division factor
    #[inline(always)]
    pub fn ic19int(&mut self) -> IC19INT_W<'_, IC19CFGRrs> {
        IC19INT_W::new(self, 16)
    }
    ///Bits 28:29 - Divider IC19 Source Selection
    #[inline(always)]
    pub fn ic19sel(&mut self) -> IC19SEL_W<'_, IC19CFGRrs> {
        IC19SEL_W::new(self, 28)
    }
}
/**RCC IC19 configuration register

You can [`read`](crate::Reg::read) this register and get [`ic19cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic19cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:IC19CFGR)*/
pub struct IC19CFGRrs;
impl crate::RegisterSpec for IC19CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`ic19cfgr::R`](R) reader structure
impl crate::Readable for IC19CFGRrs {}
///`write(|w| ..)` method takes [`ic19cfgr::W`](W) writer structure
impl crate::Writable for IC19CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IC19CFGR to value 0x3000_0000
impl crate::Resettable for IC19CFGRrs {
    const RESET_VALUE: u32 = 0x3000_0000;
}
