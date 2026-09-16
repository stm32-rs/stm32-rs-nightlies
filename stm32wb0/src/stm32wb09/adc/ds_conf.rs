///Register `DS_CONF` reader
pub type R = crate::R<DS_CONFrs>;
///Register `DS_CONF` writer
pub type W = crate::W<DS_CONFrs>;
///Field `DS_RATIO` reader - DS_RATIO\[2:0\]: program the Down Sampler ratio (N factor) 000: ratio = 1, no down sampling (default) 001: ratio = 2 010: ratio = 4 011: ratio = 8 100: ratio = 16 101: ratio = 32 110: ratio = 64 111: ratio = 128
pub type DS_RATIO_R = crate::FieldReader;
///Field `DS_RATIO` writer - DS_RATIO\[2:0\]: program the Down Sampler ratio (N factor) 000: ratio = 1, no down sampling (default) 001: ratio = 2 010: ratio = 4 011: ratio = 8 100: ratio = 16 101: ratio = 32 110: ratio = 64 111: ratio = 128
pub type DS_RATIO_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DS_WIDTH` reader - DS_WIDTH\[2:0\]: program the Down Sampler width of data output (DSDTATA) 000: DS_DATA output on 12-bit (default) 001: DS_DATA output on 13-bit 010: DS_DATA output on 14-bit 011: DS_DATA output on 15-bit 100: DS_DATA output on 16-bit 1xx: reserved
pub type DS_WIDTH_R = crate::FieldReader;
///Field `DS_WIDTH` writer - DS_WIDTH\[2:0\]: program the Down Sampler width of data output (DSDTATA) 000: DS_DATA output on 12-bit (default) 001: DS_DATA output on 13-bit 010: DS_DATA output on 14-bit 011: DS_DATA output on 15-bit 100: DS_DATA output on 16-bit 1xx: reserved
pub type DS_WIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - DS_RATIO\[2:0\]: program the Down Sampler ratio (N factor) 000: ratio = 1, no down sampling (default) 001: ratio = 2 010: ratio = 4 011: ratio = 8 100: ratio = 16 101: ratio = 32 110: ratio = 64 111: ratio = 128
    #[inline(always)]
    pub fn ds_ratio(&self) -> DS_RATIO_R {
        DS_RATIO_R::new((self.bits & 7) as u8)
    }
    ///Bits 3:5 - DS_WIDTH\[2:0\]: program the Down Sampler width of data output (DSDTATA) 000: DS_DATA output on 12-bit (default) 001: DS_DATA output on 13-bit 010: DS_DATA output on 14-bit 011: DS_DATA output on 15-bit 100: DS_DATA output on 16-bit 1xx: reserved
    #[inline(always)]
    pub fn ds_width(&self) -> DS_WIDTH_R {
        DS_WIDTH_R::new(((self.bits >> 3) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DS_CONF")
            .field("ds_ratio", &self.ds_ratio())
            .field("ds_width", &self.ds_width())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - DS_RATIO\[2:0\]: program the Down Sampler ratio (N factor) 000: ratio = 1, no down sampling (default) 001: ratio = 2 010: ratio = 4 011: ratio = 8 100: ratio = 16 101: ratio = 32 110: ratio = 64 111: ratio = 128
    #[inline(always)]
    pub fn ds_ratio(&mut self) -> DS_RATIO_W<'_, DS_CONFrs> {
        DS_RATIO_W::new(self, 0)
    }
    ///Bits 3:5 - DS_WIDTH\[2:0\]: program the Down Sampler width of data output (DSDTATA) 000: DS_DATA output on 12-bit (default) 001: DS_DATA output on 13-bit 010: DS_DATA output on 14-bit 011: DS_DATA output on 15-bit 100: DS_DATA output on 16-bit 1xx: reserved
    #[inline(always)]
    pub fn ds_width(&mut self) -> DS_WIDTH_W<'_, DS_CONFrs> {
        DS_WIDTH_W::new(self, 3)
    }
}
/**DS_CONF register

You can [`read`](crate::Reg::read) this register and get [`ds_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ds_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#ADC:DS_CONF)*/
pub struct DS_CONFrs;
impl crate::RegisterSpec for DS_CONFrs {
    type Ux = u32;
}
///`read()` method returns [`ds_conf::R`](R) reader structure
impl crate::Readable for DS_CONFrs {}
///`write(|w| ..)` method takes [`ds_conf::W`](W) writer structure
impl crate::Writable for DS_CONFrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DS_CONF to value 0
impl crate::Resettable for DS_CONFrs {}
