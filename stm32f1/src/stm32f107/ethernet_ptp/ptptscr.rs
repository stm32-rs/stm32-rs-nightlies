///Register `PTPTSCR` reader
pub type R = crate::R<PTPTSCRrs>;
///Register `PTPTSCR` writer
pub type W = crate::W<PTPTSCRrs>;
///Field `TSE` reader - Time stamp enable
pub type TSE_R = crate::BitReader;
///Field `TSE` writer - Time stamp enable
pub type TSE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSFCU` reader - Time stamp fine or coarse update
pub type TSFCU_R = crate::BitReader;
///Field `TSFCU` writer - Time stamp fine or coarse update
pub type TSFCU_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSSTI` reader - Time stamp system time initialize
pub type TSSTI_R = crate::BitReader;
///Field `TSSTI` writer - Time stamp system time initialize
pub type TSSTI_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSSTU` reader - Time stamp system time update
pub type TSSTU_R = crate::BitReader;
///Field `TSSTU` writer - Time stamp system time update
pub type TSSTU_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSITE` reader - Time stamp interrupt trigger enable
pub type TSITE_R = crate::BitReader;
///Field `TSITE` writer - Time stamp interrupt trigger enable
pub type TSITE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSARU` reader - Time stamp addend register update
pub type TSARU_R = crate::BitReader;
///Field `TSARU` writer - Time stamp addend register update
pub type TSARU_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Time stamp enable
    #[inline(always)]
    pub fn tse(&self) -> TSE_R {
        TSE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Time stamp fine or coarse update
    #[inline(always)]
    pub fn tsfcu(&self) -> TSFCU_R {
        TSFCU_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Time stamp system time initialize
    #[inline(always)]
    pub fn tssti(&self) -> TSSTI_R {
        TSSTI_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Time stamp system time update
    #[inline(always)]
    pub fn tsstu(&self) -> TSSTU_R {
        TSSTU_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Time stamp interrupt trigger enable
    #[inline(always)]
    pub fn tsite(&self) -> TSITE_R {
        TSITE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Time stamp addend register update
    #[inline(always)]
    pub fn tsaru(&self) -> TSARU_R {
        TSARU_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PTPTSCR")
            .field("tse", &self.tse())
            .field("tsfcu", &self.tsfcu())
            .field("tssti", &self.tssti())
            .field("tsstu", &self.tsstu())
            .field("tsite", &self.tsite())
            .field("tsaru", &self.tsaru())
            .finish()
    }
}
impl W {
    ///Bit 0 - Time stamp enable
    #[inline(always)]
    pub fn tse(&mut self) -> TSE_W<'_, PTPTSCRrs> {
        TSE_W::new(self, 0)
    }
    ///Bit 1 - Time stamp fine or coarse update
    #[inline(always)]
    pub fn tsfcu(&mut self) -> TSFCU_W<'_, PTPTSCRrs> {
        TSFCU_W::new(self, 1)
    }
    ///Bit 2 - Time stamp system time initialize
    #[inline(always)]
    pub fn tssti(&mut self) -> TSSTI_W<'_, PTPTSCRrs> {
        TSSTI_W::new(self, 2)
    }
    ///Bit 3 - Time stamp system time update
    #[inline(always)]
    pub fn tsstu(&mut self) -> TSSTU_W<'_, PTPTSCRrs> {
        TSSTU_W::new(self, 3)
    }
    ///Bit 4 - Time stamp interrupt trigger enable
    #[inline(always)]
    pub fn tsite(&mut self) -> TSITE_W<'_, PTPTSCRrs> {
        TSITE_W::new(self, 4)
    }
    ///Bit 5 - Time stamp addend register update
    #[inline(always)]
    pub fn tsaru(&mut self) -> TSARU_W<'_, PTPTSCRrs> {
        TSARU_W::new(self, 5)
    }
}
/**Ethernet PTP time stamp control register (ETH_PTPTSCR)

You can [`read`](crate::Reg::read) this register and get [`ptptscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptptscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F107.html#Ethernet_PTP:PTPTSCR)*/
pub struct PTPTSCRrs;
impl crate::RegisterSpec for PTPTSCRrs {
    type Ux = u32;
}
///`read()` method returns [`ptptscr::R`](R) reader structure
impl crate::Readable for PTPTSCRrs {}
///`write(|w| ..)` method takes [`ptptscr::W`](W) writer structure
impl crate::Writable for PTPTSCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PTPTSCR to value 0
impl crate::Resettable for PTPTSCRrs {}
