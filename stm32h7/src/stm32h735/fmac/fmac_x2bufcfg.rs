///Register `FMAC_X2BUFCFG` reader
pub type R = crate::R<FMAC_X2BUFCFGrs>;
///Register `FMAC_X2BUFCFG` writer
pub type W = crate::W<FMAC_X2BUFCFGrs>;
///Field `X2_BASE` reader - Base address of X2 buffer The X2 buffer base address can be modified while START=1, for example to change coefficient values. The filter should be stalled when doing this, since changing the coefficients while a calculation is ongoing affects the result.
pub type X2_BASE_R = crate::FieldReader;
///Field `X2_BASE` writer - Base address of X2 buffer The X2 buffer base address can be modified while START=1, for example to change coefficient values. The filter should be stalled when doing this, since changing the coefficients while a calculation is ongoing affects the result.
pub type X2_BASE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `X2_BUF_SIZE` reader - Size of X2 buffer in 16-bit words This bitfield can not be modified when a function is ongoing (START = 1).
pub type X2_BUF_SIZE_R = crate::FieldReader;
///Field `X2_BUF_SIZE` writer - Size of X2 buffer in 16-bit words This bitfield can not be modified when a function is ongoing (START = 1).
pub type X2_BUF_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Base address of X2 buffer The X2 buffer base address can be modified while START=1, for example to change coefficient values. The filter should be stalled when doing this, since changing the coefficients while a calculation is ongoing affects the result.
    #[inline(always)]
    pub fn x2_base(&self) -> X2_BASE_R {
        X2_BASE_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Size of X2 buffer in 16-bit words This bitfield can not be modified when a function is ongoing (START = 1).
    #[inline(always)]
    pub fn x2_buf_size(&self) -> X2_BUF_SIZE_R {
        X2_BUF_SIZE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FMAC_X2BUFCFG")
            .field("x2_base", &self.x2_base())
            .field("x2_buf_size", &self.x2_buf_size())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Base address of X2 buffer The X2 buffer base address can be modified while START=1, for example to change coefficient values. The filter should be stalled when doing this, since changing the coefficients while a calculation is ongoing affects the result.
    #[inline(always)]
    pub fn x2_base(&mut self) -> X2_BASE_W<FMAC_X2BUFCFGrs> {
        X2_BASE_W::new(self, 0)
    }
    ///Bits 8:15 - Size of X2 buffer in 16-bit words This bitfield can not be modified when a function is ongoing (START = 1).
    #[inline(always)]
    pub fn x2_buf_size(&mut self) -> X2_BUF_SIZE_W<FMAC_X2BUFCFGrs> {
        X2_BUF_SIZE_W::new(self, 8)
    }
}
/**FMAC X2 buffer configuration register

You can [`read`](crate::Reg::read) this register and get [`fmac_x2bufcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmac_x2bufcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FMAC:FMAC_X2BUFCFG)*/
pub struct FMAC_X2BUFCFGrs;
impl crate::RegisterSpec for FMAC_X2BUFCFGrs {
    type Ux = u32;
}
///`read()` method returns [`fmac_x2bufcfg::R`](R) reader structure
impl crate::Readable for FMAC_X2BUFCFGrs {}
///`write(|w| ..)` method takes [`fmac_x2bufcfg::W`](W) writer structure
impl crate::Writable for FMAC_X2BUFCFGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FMAC_X2BUFCFG to value 0
impl crate::Resettable for FMAC_X2BUFCFGrs {
    const RESET_VALUE: u32 = 0;
}