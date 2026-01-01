///Register `IC16CFGR` reader
pub type R = crate::R<IC16CFGRrs>;
///Register `IC16CFGR` writer
pub type W = crate::W<IC16CFGRrs>;
///Field `IC16INT` reader - Divider IC16 integer division factor
pub type IC16INT_R = crate::FieldReader;
///Field `IC16INT` writer - Divider IC16 integer division factor
pub type IC16INT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `IC16SEL` reader - Divider IC16 Source Selection
pub type IC16SEL_R = crate::FieldReader;
///Field `IC16SEL` writer - Divider IC16 Source Selection
pub type IC16SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 16:23 - Divider IC16 integer division factor
    #[inline(always)]
    pub fn ic16int(&self) -> IC16INT_R {
        IC16INT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 28:29 - Divider IC16 Source Selection
    #[inline(always)]
    pub fn ic16sel(&self) -> IC16SEL_R {
        IC16SEL_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IC16CFGR")
            .field("ic16int", &self.ic16int())
            .field("ic16sel", &self.ic16sel())
            .finish()
    }
}
impl W {
    ///Bits 16:23 - Divider IC16 integer division factor
    #[inline(always)]
    pub fn ic16int(&mut self) -> IC16INT_W<'_, IC16CFGRrs> {
        IC16INT_W::new(self, 16)
    }
    ///Bits 28:29 - Divider IC16 Source Selection
    #[inline(always)]
    pub fn ic16sel(&mut self) -> IC16SEL_W<'_, IC16CFGRrs> {
        IC16SEL_W::new(self, 28)
    }
}
/**RCC IC16 configuration register

You can [`read`](crate::Reg::read) this register and get [`ic16cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic16cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RCC:IC16CFGR)*/
pub struct IC16CFGRrs;
impl crate::RegisterSpec for IC16CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`ic16cfgr::R`](R) reader structure
impl crate::Readable for IC16CFGRrs {}
///`write(|w| ..)` method takes [`ic16cfgr::W`](W) writer structure
impl crate::Writable for IC16CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IC16CFGR to value 0x3000_0000
impl crate::Resettable for IC16CFGRrs {
    const RESET_VALUE: u32 = 0x3000_0000;
}
