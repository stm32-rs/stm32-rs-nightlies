///Register `SQR1` reader
pub type R = crate::R<SQR1rs>;
///Register `SQR1` writer
pub type W = crate::W<SQR1rs>;
///Field `SQ25` reader - 25th conversion in regular sequence
pub type SQ25_R = crate::FieldReader;
///Field `SQ25` writer - 25th conversion in regular sequence
pub type SQ25_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `SQ26` reader - 26th conversion in regular sequence
pub type SQ26_R = crate::FieldReader;
///Field `SQ26` writer - 26th conversion in regular sequence
pub type SQ26_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `SQ27` reader - 27th conversion in regular sequence
pub type SQ27_R = crate::FieldReader;
///Field `SQ27` writer - 27th conversion in regular sequence
pub type SQ27_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `SQ28` reader - 28th conversion in regular sequence
pub type SQ28_R = crate::FieldReader;
///Field `SQ28` writer - 28th conversion in regular sequence
pub type SQ28_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `L` reader - Regular channel sequence length
pub type L_R = crate::FieldReader;
///Field `L` writer - Regular channel sequence length
pub type L_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:4 - 25th conversion in regular sequence
    #[inline(always)]
    pub fn sq25(&self) -> SQ25_R {
        SQ25_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 5:9 - 26th conversion in regular sequence
    #[inline(always)]
    pub fn sq26(&self) -> SQ26_R {
        SQ26_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    ///Bits 10:14 - 27th conversion in regular sequence
    #[inline(always)]
    pub fn sq27(&self) -> SQ27_R {
        SQ27_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    ///Bits 15:19 - 28th conversion in regular sequence
    #[inline(always)]
    pub fn sq28(&self) -> SQ28_R {
        SQ28_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    ///Bits 20:23 - Regular channel sequence length
    #[inline(always)]
    pub fn l(&self) -> L_R {
        L_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SQR1")
            .field("l", &self.l())
            .field("sq28", &self.sq28())
            .field("sq27", &self.sq27())
            .field("sq26", &self.sq26())
            .field("sq25", &self.sq25())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - 25th conversion in regular sequence
    #[inline(always)]
    pub fn sq25(&mut self) -> SQ25_W<SQR1rs> {
        SQ25_W::new(self, 0)
    }
    ///Bits 5:9 - 26th conversion in regular sequence
    #[inline(always)]
    pub fn sq26(&mut self) -> SQ26_W<SQR1rs> {
        SQ26_W::new(self, 5)
    }
    ///Bits 10:14 - 27th conversion in regular sequence
    #[inline(always)]
    pub fn sq27(&mut self) -> SQ27_W<SQR1rs> {
        SQ27_W::new(self, 10)
    }
    ///Bits 15:19 - 28th conversion in regular sequence
    #[inline(always)]
    pub fn sq28(&mut self) -> SQ28_W<SQR1rs> {
        SQ28_W::new(self, 15)
    }
    ///Bits 20:23 - Regular channel sequence length
    #[inline(always)]
    pub fn l(&mut self) -> L_W<SQR1rs> {
        L_W::new(self, 20)
    }
}
/**regular sequence register 1

You can [`read`](crate::Reg::read) this register and get [`sqr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sqr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L100.html#ADC:SQR1)*/
pub struct SQR1rs;
impl crate::RegisterSpec for SQR1rs {
    type Ux = u32;
}
///`read()` method returns [`sqr1::R`](R) reader structure
impl crate::Readable for SQR1rs {}
///`write(|w| ..)` method takes [`sqr1::W`](W) writer structure
impl crate::Writable for SQR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SQR1 to value 0
impl crate::Resettable for SQR1rs {
    const RESET_VALUE: u32 = 0;
}
