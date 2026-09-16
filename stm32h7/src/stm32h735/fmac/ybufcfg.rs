///Register `YBUFCFG` reader
pub type R = crate::R<YBUFCFGrs>;
///Register `YBUFCFG` writer
pub type W = crate::W<YBUFCFGrs>;
///Field `Y_BASE` reader - Base address of Y buffer
pub type Y_BASE_R = crate::FieldReader;
///Field `Y_BASE` writer - Base address of Y buffer
pub type Y_BASE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `Y_BUF_SIZE` reader - Size of Y buffer in 16-bit words For FIR filters, the minimum buffer size is 1 (+ the watermark threshold). For IIR filters the minimum buffer size is the number of feedback taps (+ the watermark threshold).
pub type Y_BUF_SIZE_R = crate::FieldReader;
///Field `Y_BUF_SIZE` writer - Size of Y buffer in 16-bit words For FIR filters, the minimum buffer size is 1 (+ the watermark threshold). For IIR filters the minimum buffer size is the number of feedback taps (+ the watermark threshold).
pub type Y_BUF_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `EMPTY_WM` reader - Watermark for buffer empty flag Defines the threshold for setting the Y buffer empty flag when operating in circular mode. The flag is set if the number of unread values in the buffer is less than 2EMPTY_WM. 2: Threshold = 4 3: Threshold = 8 Setting a threshold greater than 1 allows several data to be transferred from the buffer under one interrupt. Threshold should be set to 1 if DMA read requests are enabled (DMAREN = 1 in FMAC_CR register).
pub type EMPTY_WM_R = crate::FieldReader;
///Field `EMPTY_WM` writer - Watermark for buffer empty flag Defines the threshold for setting the Y buffer empty flag when operating in circular mode. The flag is set if the number of unread values in the buffer is less than 2EMPTY_WM. 2: Threshold = 4 3: Threshold = 8 Setting a threshold greater than 1 allows several data to be transferred from the buffer under one interrupt. Threshold should be set to 1 if DMA read requests are enabled (DMAREN = 1 in FMAC_CR register).
pub type EMPTY_WM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:7 - Base address of Y buffer
    #[inline(always)]
    pub fn y_base(&self) -> Y_BASE_R {
        Y_BASE_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Size of Y buffer in 16-bit words For FIR filters, the minimum buffer size is 1 (+ the watermark threshold). For IIR filters the minimum buffer size is the number of feedback taps (+ the watermark threshold).
    #[inline(always)]
    pub fn y_buf_size(&self) -> Y_BUF_SIZE_R {
        Y_BUF_SIZE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 24:25 - Watermark for buffer empty flag Defines the threshold for setting the Y buffer empty flag when operating in circular mode. The flag is set if the number of unread values in the buffer is less than 2EMPTY_WM. 2: Threshold = 4 3: Threshold = 8 Setting a threshold greater than 1 allows several data to be transferred from the buffer under one interrupt. Threshold should be set to 1 if DMA read requests are enabled (DMAREN = 1 in FMAC_CR register).
    #[inline(always)]
    pub fn empty_wm(&self) -> EMPTY_WM_R {
        EMPTY_WM_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("YBUFCFG")
            .field("y_base", &self.y_base())
            .field("y_buf_size", &self.y_buf_size())
            .field("empty_wm", &self.empty_wm())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Base address of Y buffer
    #[inline(always)]
    pub fn y_base(&mut self) -> Y_BASE_W<'_, YBUFCFGrs> {
        Y_BASE_W::new(self, 0)
    }
    ///Bits 8:15 - Size of Y buffer in 16-bit words For FIR filters, the minimum buffer size is 1 (+ the watermark threshold). For IIR filters the minimum buffer size is the number of feedback taps (+ the watermark threshold).
    #[inline(always)]
    pub fn y_buf_size(&mut self) -> Y_BUF_SIZE_W<'_, YBUFCFGrs> {
        Y_BUF_SIZE_W::new(self, 8)
    }
    ///Bits 24:25 - Watermark for buffer empty flag Defines the threshold for setting the Y buffer empty flag when operating in circular mode. The flag is set if the number of unread values in the buffer is less than 2EMPTY_WM. 2: Threshold = 4 3: Threshold = 8 Setting a threshold greater than 1 allows several data to be transferred from the buffer under one interrupt. Threshold should be set to 1 if DMA read requests are enabled (DMAREN = 1 in FMAC_CR register).
    #[inline(always)]
    pub fn empty_wm(&mut self) -> EMPTY_WM_W<'_, YBUFCFGrs> {
        EMPTY_WM_W::new(self, 24)
    }
}
/**FMAC Y buffer configuration register

You can [`read`](crate::Reg::read) this register and get [`ybufcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ybufcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FMAC:YBUFCFG)*/
pub struct YBUFCFGrs;
impl crate::RegisterSpec for YBUFCFGrs {
    type Ux = u32;
}
///`read()` method returns [`ybufcfg::R`](R) reader structure
impl crate::Readable for YBUFCFGrs {}
///`write(|w| ..)` method takes [`ybufcfg::W`](W) writer structure
impl crate::Writable for YBUFCFGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets YBUFCFG to value 0
impl crate::Resettable for YBUFCFGrs {}
