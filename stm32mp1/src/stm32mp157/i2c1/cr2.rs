///Register `CR2` reader
pub type R = crate::R<CR2rs>;
///Register `CR2` writer
pub type W = crate::W<CR2rs>;
///Field `SADD` reader - SADD
pub type SADD_R = crate::FieldReader<u16>;
///Field `SADD` writer - SADD
pub type SADD_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `RD_WRN` reader - RD_WRN
pub type RD_WRN_R = crate::BitReader;
///Field `RD_WRN` writer - RD_WRN
pub type RD_WRN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADD10` reader - ADD10
pub type ADD10_R = crate::BitReader;
///Field `ADD10` writer - ADD10
pub type ADD10_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HEAD10R` reader - HEAD10R
pub type HEAD10R_R = crate::BitReader;
///Field `HEAD10R` writer - HEAD10R
pub type HEAD10R_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `START` reader - START
pub type START_R = crate::BitReader;
///Field `START` writer - START
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STOP` reader - STOP
pub type STOP_R = crate::BitReader;
///Field `STOP` writer - STOP
pub type STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NACK` reader - NACK
pub type NACK_R = crate::BitReader;
///Field `NACK` writer - NACK
pub type NACK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NBYTES` reader - NBYTES
pub type NBYTES_R = crate::FieldReader;
///Field `NBYTES` writer - NBYTES
pub type NBYTES_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `RELOAD` reader - RELOAD
pub type RELOAD_R = crate::BitReader;
///Field `RELOAD` writer - RELOAD
pub type RELOAD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AUTOEND` reader - AUTOEND
pub type AUTOEND_R = crate::BitReader;
///Field `AUTOEND` writer - AUTOEND
pub type AUTOEND_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PECBYTE` reader - PECBYTE
pub type PECBYTE_R = crate::BitReader;
///Field `PECBYTE` writer - PECBYTE
pub type PECBYTE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:9 - SADD
    #[inline(always)]
    pub fn sadd(&self) -> SADD_R {
        SADD_R::new((self.bits & 0x03ff) as u16)
    }
    ///Bit 10 - RD_WRN
    #[inline(always)]
    pub fn rd_wrn(&self) -> RD_WRN_R {
        RD_WRN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - ADD10
    #[inline(always)]
    pub fn add10(&self) -> ADD10_R {
        ADD10_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - HEAD10R
    #[inline(always)]
    pub fn head10r(&self) -> HEAD10R_R {
        HEAD10R_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - START
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - STOP
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - NACK
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:23 - NBYTES
    #[inline(always)]
    pub fn nbytes(&self) -> NBYTES_R {
        NBYTES_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bit 24 - RELOAD
    #[inline(always)]
    pub fn reload(&self) -> RELOAD_R {
        RELOAD_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - AUTOEND
    #[inline(always)]
    pub fn autoend(&self) -> AUTOEND_R {
        AUTOEND_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - PECBYTE
    #[inline(always)]
    pub fn pecbyte(&self) -> PECBYTE_R {
        PECBYTE_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR2")
            .field("sadd", &self.sadd())
            .field("rd_wrn", &self.rd_wrn())
            .field("add10", &self.add10())
            .field("head10r", &self.head10r())
            .field("start", &self.start())
            .field("stop", &self.stop())
            .field("nack", &self.nack())
            .field("nbytes", &self.nbytes())
            .field("reload", &self.reload())
            .field("autoend", &self.autoend())
            .field("pecbyte", &self.pecbyte())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - SADD
    #[inline(always)]
    pub fn sadd(&mut self) -> SADD_W<'_, CR2rs> {
        SADD_W::new(self, 0)
    }
    ///Bit 10 - RD_WRN
    #[inline(always)]
    pub fn rd_wrn(&mut self) -> RD_WRN_W<'_, CR2rs> {
        RD_WRN_W::new(self, 10)
    }
    ///Bit 11 - ADD10
    #[inline(always)]
    pub fn add10(&mut self) -> ADD10_W<'_, CR2rs> {
        ADD10_W::new(self, 11)
    }
    ///Bit 12 - HEAD10R
    #[inline(always)]
    pub fn head10r(&mut self) -> HEAD10R_W<'_, CR2rs> {
        HEAD10R_W::new(self, 12)
    }
    ///Bit 13 - START
    #[inline(always)]
    pub fn start(&mut self) -> START_W<'_, CR2rs> {
        START_W::new(self, 13)
    }
    ///Bit 14 - STOP
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W<'_, CR2rs> {
        STOP_W::new(self, 14)
    }
    ///Bit 15 - NACK
    #[inline(always)]
    pub fn nack(&mut self) -> NACK_W<'_, CR2rs> {
        NACK_W::new(self, 15)
    }
    ///Bits 16:23 - NBYTES
    #[inline(always)]
    pub fn nbytes(&mut self) -> NBYTES_W<'_, CR2rs> {
        NBYTES_W::new(self, 16)
    }
    ///Bit 24 - RELOAD
    #[inline(always)]
    pub fn reload(&mut self) -> RELOAD_W<'_, CR2rs> {
        RELOAD_W::new(self, 24)
    }
    ///Bit 25 - AUTOEND
    #[inline(always)]
    pub fn autoend(&mut self) -> AUTOEND_W<'_, CR2rs> {
        AUTOEND_W::new(self, 25)
    }
    ///Bit 26 - PECBYTE
    #[inline(always)]
    pub fn pecbyte(&mut self) -> PECBYTE_W<'_, CR2rs> {
        PECBYTE_W::new(self, 26)
    }
}
/**Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x i2c_pclk + 6 x i2c_ker_ck.

You can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#I2C1:CR2)*/
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u32;
}
///`read()` method returns [`cr2::R`](R) reader structure
impl crate::Readable for CR2rs {}
///`write(|w| ..)` method takes [`cr2::W`](W) writer structure
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR2 to value 0
impl crate::Resettable for CR2rs {}
