///Register `IC18CFGR` reader
pub type R = crate::R<IC18CFGRrs>;
///Register `IC18CFGR` writer
pub type W = crate::W<IC18CFGRrs>;
///Field `IC18INT` reader - Divider IC18 integer division factor
pub type IC18INT_R = crate::FieldReader;
///Field `IC18INT` writer - Divider IC18 integer division factor
pub type IC18INT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `IC18SEL` reader - Divider IC18 Source Selection
pub type IC18SEL_R = crate::FieldReader;
///Field `IC18SEL` writer - Divider IC18 Source Selection
pub type IC18SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 16:23 - Divider IC18 integer division factor
    #[inline(always)]
    pub fn ic18int(&self) -> IC18INT_R {
        IC18INT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 28:29 - Divider IC18 Source Selection
    #[inline(always)]
    pub fn ic18sel(&self) -> IC18SEL_R {
        IC18SEL_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IC18CFGR")
            .field("ic18int", &self.ic18int())
            .field("ic18sel", &self.ic18sel())
            .finish()
    }
}
impl W {
    ///Bits 16:23 - Divider IC18 integer division factor
    #[inline(always)]
    pub fn ic18int(&mut self) -> IC18INT_W<'_, IC18CFGRrs> {
        IC18INT_W::new(self, 16)
    }
    ///Bits 28:29 - Divider IC18 Source Selection
    #[inline(always)]
    pub fn ic18sel(&mut self) -> IC18SEL_W<'_, IC18CFGRrs> {
        IC18SEL_W::new(self, 28)
    }
}
/**RCC IC18 configuration register

You can [`read`](crate::Reg::read) this register and get [`ic18cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic18cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RCC:IC18CFGR)*/
pub struct IC18CFGRrs;
impl crate::RegisterSpec for IC18CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`ic18cfgr::R`](R) reader structure
impl crate::Readable for IC18CFGRrs {}
///`write(|w| ..)` method takes [`ic18cfgr::W`](W) writer structure
impl crate::Writable for IC18CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IC18CFGR to value 0x3000_0000
impl crate::Resettable for IC18CFGRrs {
    const RESET_VALUE: u32 = 0x3000_0000;
}
