///Register `ICSR` reader
pub type R = crate::R<ICSRrs>;
///Register `ICSR` writer
pub type W = crate::W<ICSRrs>;
///Field `WUTWF` reader - Wake-up timer write flag
pub type WUTWF_R = crate::BitReader;
///Field `SHPF` reader - Shift operation pending
pub type SHPF_R = crate::BitReader;
///Field `INITS` reader - Initialization status flag
pub type INITS_R = crate::BitReader;
///Field `RSF` reader - Registers synchronization flag
pub type RSF_R = crate::BitReader;
///Field `RSF` writer - Registers synchronization flag
pub type RSF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INITF` reader - Initialization flag
pub type INITF_R = crate::BitReader;
///Field `INIT` reader - Initialization mode
pub type INIT_R = crate::BitReader;
///Field `INIT` writer - Initialization mode
pub type INIT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BIN` reader - Binary mode
pub type BIN_R = crate::FieldReader;
///Field `BIN` writer - Binary mode
pub type BIN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `BCDU` reader - BCD update (BIN = 10 or 11)
pub type BCDU_R = crate::FieldReader;
///Field `BCDU` writer - BCD update (BIN = 10 or 11)
pub type BCDU_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `RECALPF` reader - Recalibration pending Flag
pub type RECALPF_R = crate::BitReader;
impl R {
    ///Bit 2 - Wake-up timer write flag
    #[inline(always)]
    pub fn wutwf(&self) -> WUTWF_R {
        WUTWF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Shift operation pending
    #[inline(always)]
    pub fn shpf(&self) -> SHPF_R {
        SHPF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Initialization status flag
    #[inline(always)]
    pub fn inits(&self) -> INITS_R {
        INITS_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Registers synchronization flag
    #[inline(always)]
    pub fn rsf(&self) -> RSF_R {
        RSF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Initialization flag
    #[inline(always)]
    pub fn initf(&self) -> INITF_R {
        INITF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Initialization mode
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:9 - Binary mode
    #[inline(always)]
    pub fn bin(&self) -> BIN_R {
        BIN_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:12 - BCD update (BIN = 10 or 11)
    #[inline(always)]
    pub fn bcdu(&self) -> BCDU_R {
        BCDU_R::new(((self.bits >> 10) & 7) as u8)
    }
    ///Bit 16 - Recalibration pending Flag
    #[inline(always)]
    pub fn recalpf(&self) -> RECALPF_R {
        RECALPF_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICSR")
            .field("wutwf", &self.wutwf())
            .field("shpf", &self.shpf())
            .field("inits", &self.inits())
            .field("rsf", &self.rsf())
            .field("initf", &self.initf())
            .field("init", &self.init())
            .field("bin", &self.bin())
            .field("bcdu", &self.bcdu())
            .field("recalpf", &self.recalpf())
            .finish()
    }
}
impl W {
    ///Bit 5 - Registers synchronization flag
    #[inline(always)]
    pub fn rsf(&mut self) -> RSF_W<'_, ICSRrs> {
        RSF_W::new(self, 5)
    }
    ///Bit 7 - Initialization mode
    #[inline(always)]
    pub fn init(&mut self) -> INIT_W<'_, ICSRrs> {
        INIT_W::new(self, 7)
    }
    ///Bits 8:9 - Binary mode
    #[inline(always)]
    pub fn bin(&mut self) -> BIN_W<'_, ICSRrs> {
        BIN_W::new(self, 8)
    }
    ///Bits 10:12 - BCD update (BIN = 10 or 11)
    #[inline(always)]
    pub fn bcdu(&mut self) -> BCDU_W<'_, ICSRrs> {
        BCDU_W::new(self, 10)
    }
}
/**RTC initialization control and status register

You can [`read`](crate::Reg::read) this register and get [`icsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RTC:ICSR)*/
pub struct ICSRrs;
impl crate::RegisterSpec for ICSRrs {
    type Ux = u32;
}
///`read()` method returns [`icsr::R`](R) reader structure
impl crate::Readable for ICSRrs {}
///`write(|w| ..)` method takes [`icsr::W`](W) writer structure
impl crate::Writable for ICSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICSR to value 0x07
impl crate::Resettable for ICSRrs {
    const RESET_VALUE: u32 = 0x07;
}
