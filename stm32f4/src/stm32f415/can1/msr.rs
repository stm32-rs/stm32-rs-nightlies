///Register `MSR` reader
pub type R = crate::R<MSRrs>;
///Register `MSR` writer
pub type W = crate::W<MSRrs>;
///Field `INAK` reader - INAK
pub type INAK_R = crate::BitReader;
///Field `SLAK` reader - SLAK
pub type SLAK_R = crate::BitReader;
///Field `ERRI` reader - ERRI
pub type ERRI_R = crate::BitReader;
///Field `ERRI` writer - ERRI
pub type ERRI_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUI` reader - WKUI
pub type WKUI_R = crate::BitReader;
///Field `WKUI` writer - WKUI
pub type WKUI_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SLAKI` reader - SLAKI
pub type SLAKI_R = crate::BitReader;
///Field `SLAKI` writer - SLAKI
pub type SLAKI_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXM` reader - TXM
pub type TXM_R = crate::BitReader;
///Field `RXM` reader - RXM
pub type RXM_R = crate::BitReader;
///Field `SAMP` reader - SAMP
pub type SAMP_R = crate::BitReader;
///Field `RX` reader - RX
pub type RX_R = crate::BitReader;
impl R {
    ///Bit 0 - INAK
    #[inline(always)]
    pub fn inak(&self) -> INAK_R {
        INAK_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SLAK
    #[inline(always)]
    pub fn slak(&self) -> SLAK_R {
        SLAK_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - ERRI
    #[inline(always)]
    pub fn erri(&self) -> ERRI_R {
        ERRI_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - WKUI
    #[inline(always)]
    pub fn wkui(&self) -> WKUI_R {
        WKUI_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - SLAKI
    #[inline(always)]
    pub fn slaki(&self) -> SLAKI_R {
        SLAKI_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - TXM
    #[inline(always)]
    pub fn txm(&self) -> TXM_R {
        TXM_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - RXM
    #[inline(always)]
    pub fn rxm(&self) -> RXM_R {
        RXM_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - SAMP
    #[inline(always)]
    pub fn samp(&self) -> SAMP_R {
        SAMP_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - RX
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MSR")
            .field("rx", &self.rx())
            .field("samp", &self.samp())
            .field("rxm", &self.rxm())
            .field("txm", &self.txm())
            .field("slaki", &self.slaki())
            .field("wkui", &self.wkui())
            .field("erri", &self.erri())
            .field("slak", &self.slak())
            .field("inak", &self.inak())
            .finish()
    }
}
impl W {
    ///Bit 2 - ERRI
    #[inline(always)]
    pub fn erri(&mut self) -> ERRI_W<'_, MSRrs> {
        ERRI_W::new(self, 2)
    }
    ///Bit 3 - WKUI
    #[inline(always)]
    pub fn wkui(&mut self) -> WKUI_W<'_, MSRrs> {
        WKUI_W::new(self, 3)
    }
    ///Bit 4 - SLAKI
    #[inline(always)]
    pub fn slaki(&mut self) -> SLAKI_W<'_, MSRrs> {
        SLAKI_W::new(self, 4)
    }
}
/**master status register

You can [`read`](crate::Reg::read) this register and get [`msr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`msr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#CAN1:MSR)*/
pub struct MSRrs;
impl crate::RegisterSpec for MSRrs {
    type Ux = u32;
}
///`read()` method returns [`msr::R`](R) reader structure
impl crate::Readable for MSRrs {}
///`write(|w| ..)` method takes [`msr::W`](W) writer structure
impl crate::Writable for MSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MSR to value 0x0c02
impl crate::Resettable for MSRrs {
    const RESET_VALUE: u32 = 0x0c02;
}
