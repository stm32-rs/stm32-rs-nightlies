///Register `IC12CFGR` reader
pub type R = crate::R<IC12CFGRrs>;
///Register `IC12CFGR` writer
pub type W = crate::W<IC12CFGRrs>;
///Field `IC12INT` reader - Divider IC12 integer division factor
pub type IC12INT_R = crate::FieldReader;
///Field `IC12INT` writer - Divider IC12 integer division factor
pub type IC12INT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `IC12SEL` reader - Divider IC12 Source Selection
pub type IC12SEL_R = crate::FieldReader;
///Field `IC12SEL` writer - Divider IC12 Source Selection
pub type IC12SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 16:23 - Divider IC12 integer division factor
    #[inline(always)]
    pub fn ic12int(&self) -> IC12INT_R {
        IC12INT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 28:29 - Divider IC12 Source Selection
    #[inline(always)]
    pub fn ic12sel(&self) -> IC12SEL_R {
        IC12SEL_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IC12CFGR")
            .field("ic12int", &self.ic12int())
            .field("ic12sel", &self.ic12sel())
            .finish()
    }
}
impl W {
    ///Bits 16:23 - Divider IC12 integer division factor
    #[inline(always)]
    pub fn ic12int(&mut self) -> IC12INT_W<'_, IC12CFGRrs> {
        IC12INT_W::new(self, 16)
    }
    ///Bits 28:29 - Divider IC12 Source Selection
    #[inline(always)]
    pub fn ic12sel(&mut self) -> IC12SEL_W<'_, IC12CFGRrs> {
        IC12SEL_W::new(self, 28)
    }
}
/**RCC IC12 configuration register

You can [`read`](crate::Reg::read) this register and get [`ic12cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic12cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:IC12CFGR)*/
pub struct IC12CFGRrs;
impl crate::RegisterSpec for IC12CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`ic12cfgr::R`](R) reader structure
impl crate::Readable for IC12CFGRrs {}
///`write(|w| ..)` method takes [`ic12cfgr::W`](W) writer structure
impl crate::Writable for IC12CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IC12CFGR to value 0x2000_0000
impl crate::Resettable for IC12CFGRrs {
    const RESET_VALUE: u32 = 0x2000_0000;
}
